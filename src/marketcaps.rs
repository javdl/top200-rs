// SPDX-FileCopyrightText: 2025 Joost van der Laan
// SPDX-License-Identifier: AGPL-3.0-only

use crate::api;
use crate::config;
use crate::currencies::{convert_currency, get_rate_map_from_db, update_currencies};
use crate::exchange_rates;
use crate::models;
use crate::ticker_details::{self, TickerDetails};
use anyhow::Result;
use chrono::Local;
use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use tokio_postgres::Client;
use std::collections::HashMap;
use std::sync::Arc;

/// Store market cap data in the database
async fn store_market_cap(
    client: &mut Client,
    details: &models::Details,
    rate_map: &HashMap<String, f64>,
    api_timestamp_val: i64, // Renamed from 'timestamp' to match table column logic
) -> Result<()> {
    let original_market_cap = details.market_cap.unwrap_or(0.0); // Keep as f64 for conversion
    let currency = details.currency_symbol.clone().unwrap_or_default();

    // Market cap conversions
    let eur_market_cap = convert_currency(original_market_cap, &currency, "EUR", rate_map);
    let usd_market_cap = convert_currency(original_market_cap, &currency, "USD", rate_map);

    let name = details.name.as_deref().unwrap_or("");

    // Determine exchange value.
    let exchange_val = details
        .extra
        .get("exchangeShortName") // Prefer FMP like field name
        .or_else(|| details.extra.get("exchange")) // Fallback
        .or_else(|| details.extra.get("primary_exchange")) // Polygon like field name in 'extra'
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let active = details.active.unwrap_or(true);

    client
        .execute(
            r#"
            INSERT INTO market_caps (
                ticker, name, market_cap_original, original_currency, 
                market_cap_eur, market_cap_usd, exchange, active, api_timestamp,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())
            ON CONFLICT (ticker, api_timestamp) DO UPDATE SET
                name = EXCLUDED.name,
                market_cap_original = EXCLUDED.market_cap_original,
                original_currency = EXCLUDED.original_currency,
                market_cap_eur = EXCLUDED.market_cap_eur,
                market_cap_usd = EXCLUDED.market_cap_usd,
                exchange = EXCLUDED.exchange,
                active = EXCLUDED.active,
                updated_at = NOW()
            "#,
            &[
                &details.ticker,
                &name,
                &(original_market_cap as i64), // Cast to i64 for DB
                &currency,
                &(eur_market_cap as i64),    // Cast to i64 for DB
                &(usd_market_cap as i64),    // Cast to i64 for DB
                &exchange_val,
                &active,
                &api_timestamp_val,
            ],
        )
        .await?;

    let ticker_details_obj = TickerDetails {
        ticker: details.ticker.clone(),
        description: details.description.clone(),
        homepage_url: details.homepage_url.clone(),
        employees: details.employees.clone(),
    };
    // ticker_details::update_ticker_details now takes &mut Client
    ticker_details::update_ticker_details(client, &ticker_details_obj).await?;

    Ok(())
}

/// Fetch market cap data from the database
async fn get_market_caps(client: &Client) -> Result<Vec<(f64, Vec<String>)>> {
    // Selects the latest snapshot based on the maximum api_timestamp found in the market_caps table.
    // Then joins with company_details to get descriptive fields.
    let rows = client
        .query(
            r#"
            WITH latest_snapshot AS (
                SELECT MAX(api_timestamp) as max_ts FROM market_caps
            )
            SELECT
                m.ticker,
                m.name,
                m.market_cap_original,
                m.original_currency,
                m.market_cap_eur,
                m.market_cap_usd,
                m.exchange,
                m.active,
                m.api_timestamp,
                cd.description,
                cd.homepage_url,
                cd.employees
            FROM market_caps m
            JOIN latest_snapshot ON m.api_timestamp = latest_snapshot.max_ts
            LEFT JOIN company_details cd ON m.ticker = cd.ticker
            ORDER BY m.market_cap_eur DESC NULLS LAST
            "#,
            &[],
        )
        .await?;

    let mut results = Vec::new();
    for row in rows {
        let market_cap_eur_val: i64 = row.try_get("market_cap_eur").unwrap_or(0);

        let record_vec = vec![
            row.try_get("ticker").unwrap_or_else(|_| String::new()), // Symbol
            row.try_get("ticker").unwrap_or_else(|_| String::new()), // Ticker
            row.try_get("name").unwrap_or_else(|_| String::new()),   // Name
            row.try_get::<_, Option<i64>>("market_cap_original")?.map(|val| val.to_string()).unwrap_or_default(),
            row.try_get::<_, Option<String>>("original_currency")?.unwrap_or_default(),
            market_cap_eur_val.to_string(),
            row.try_get::<_, Option<i64>>("market_cap_usd")?.map(|val| val.to_string()).unwrap_or_default(),
            row.try_get::<_, Option<String>>("exchange")?.unwrap_or_default(),
            row.try_get::<_, Option<bool>>("active")?.map(|val| val.to_string()).unwrap_or_default(),
            row.try_get::<_, Option<String>>("description")?.unwrap_or_default(),
            row.try_get::<_, Option<String>>("homepage_url")?.unwrap_or_default(),
            row.try_get::<_, Option<String>>("employees")?.unwrap_or_default(),
            row.try_get::<_, Option<i64>>("api_timestamp")?.map(|val| val.to_string()).unwrap_or_default(),
        ];
        results.push((market_cap_eur_val as f64, record_vec));
    }

    Ok(results)
}

/// Update market cap data in the database
async fn update_market_caps(client: &mut Client) -> Result<()> {
    let config = config::load_config()?;
    let tickers = [config.non_us_tickers, config.us_tickers].concat();

    // Get latest exchange rates from database
    println!("Fetching current exchange rates from database...");
    let rate_map = get_rate_map_from_db(client).await?;
    println!("✅ Exchange rates fetched from database");

    // Get FMP client for market data
    let api_key = std::env::var("FINANCIALMODELINGPREP_API_KEY")
        .expect("FINANCIALMODELINGPREP_API_KEY must be set");
    let fmp_client = Arc::new(api::FMPClient::new(api_key));

    // Create a rate_map Arc for sharing between tasks
    let rate_map_arc = Arc::new(rate_map); // Renamed to avoid shadowing, and for Arc clarity
    let total_tickers = tickers.len();

    // Use a single timestamp for all records
    let current_api_timestamp = Local::now().naive_utc().and_utc().timestamp();

    // Process tickers with progress tracking
    let progress = ProgressBar::new(total_tickers as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Update market cap data in database
    println!("Updating market cap data in database for {} tickers...", total_tickers);
    let mut failed_tickers_info = Vec::new(); // Renamed for clarity
    // NOTE: The original code iterates sequentially.
    // If fmp_client.get_details is long-running per ticker,
    // consider using tokio::spawn for concurrency here, similar to details_eu_fmp.rs.
    // For now, maintaining sequential iteration to match original logic.
    for ticker_str in &tickers { // Renamed for clarity
        let rate_map_ref = Arc::clone(&rate_map_arc); // Use Arc for rate_map
        let fmp_client_ref = Arc::clone(&fmp_client); // Use Arc for fmp_client

        progress.set_message(format!("Processing {}...", ticker_str));
        match fmp_client_ref.get_details(ticker_str, &rate_map_ref).await {
            Ok(details) => {
                // store_market_cap now takes &mut Client
                if let Err(e) = store_market_cap(client, &details, &rate_map_ref, current_api_timestamp).await {
                    eprintln!("Failed to store market cap for {}: {}", ticker_str, e);
                    failed_tickers_info.push((ticker_str.to_string(), format!("Failed to store: {}", e)));
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch details for {}: {}", ticker_str, e);
                failed_tickers_info.push((ticker_str.to_string(), format!("Failed to fetch: {}", e)));
            }
        }
        progress.inc(1);
    }
    progress.finish_with_message("All tickers processed.");

    // Print summary of failed tickers
    if !failed_tickers_info.is_empty() {
        println!("\nFailed to process {} tickers:", failed_tickers_info.len());
        for (ticker, error) in &failed_tickers_info {
            println!("  {} - {}", ticker, error);
        }
    }

    println!(
        "✅ Market cap data updated in database ({} successful, {} failed)",
        total_tickers - failed_tickers_info.len(),
        failed_tickers_info.len()
    );

    Ok(())
}

/// Export market cap data to CSV
pub async fn export_market_caps(client: &Client) -> Result<()> {
    // Get market cap data from database
    println!("Fetching market cap data from database...");
    let mut results = get_market_caps(client).await?;
    println!("✅ Market cap data fetched from database");

    // Sort by EUR market cap
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Export to CSV
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("output/combined_marketcaps_{}.csv", timestamp);
    let file = std::fs::File::create(&filename)?;
    let mut writer = Writer::from_writer(file);

    // Write headers
    writer.write_record(&[
        "Symbol",
        "Ticker",
        "Name",
        "Market Cap (Original)",
        "Original Currency",
        "Market Cap (EUR)",
        "Market Cap (USD)",
        "Exchange",
        "Active",
        "Description",
        "Homepage URL",
        "Employees",
        "Timestamp",
    ])?;

    // Write data
    for (_, record) in &results {
        writer.write_record(record)?;
    }

    println!("✅ Market cap data exported to {}", filename);
    Ok(())
}

/// Export top 100 active companies to CSV
pub async fn export_top_100_active(client: &Client) -> Result<()> {
    // Get market cap data from database
    let mut results = get_market_caps(client).await?;

    // Sort by EUR market cap
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Filter for active companies first, then take top 100
    // Assumes active is at index 8 of the inner Vec<String> from get_market_caps
    let active_results: Vec<_> = results
        .iter()
        .filter(|(_, record)| record.get(8).map_or(false, |s| s == "true"))
        .take(100)
        .collect();

    // Export to CSV
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("output/top_100_active_{}.csv", timestamp);
    let file = std::fs::File::create(&filename)?;
    let mut writer = Writer::from_writer(file);

    // Write headers
    writer.write_record(&[
        "Symbol",
        "Ticker",
        "Name",
        "Market Cap (Original)",
        "Original Currency",
        "Market Cap (EUR)",
        "Market Cap (USD)",
        "Exchange",
        "Active",
        "Description",
        "Homepage URL",
        "Employees",
        "Timestamp",
    ])?;

    // Write data
    for (_, record) in active_results {
        writer.write_record(record)?;
    }

    println!("✅ Top 100 active companies exported to {}", filename);
    Ok(())
}

/// Main entry point for market cap functionality
pub async fn marketcaps(client: &mut Client) -> Result<()> {
    // First update currencies and exchange rates
    let api_key = std::env::var("FINANCIALMODELINGPREP_API_KEY")
        .expect("FINANCIALMODELINGPREP_API_KEY must be set");
    let fmp_client = api::FMPClient::new(api_key);

    println!("Updating currencies and exchange rates...");
    // update_currencies and update_exchange_rates now take &mut Client
    update_currencies(&fmp_client, client).await?;
    exchange_rates::update_exchange_rates(&fmp_client, client).await?;

    // Then update market caps
    // update_market_caps now takes &mut Client
    update_market_caps(client).await?;

    // Export both the full list and top 100 active
    // export functions take &Client, &mut Client will deref to &Client
    export_market_caps(client).await?;
    export_top_100_active(client).await?;

    Ok(())
}
