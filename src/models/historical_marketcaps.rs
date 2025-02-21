// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::api;
use crate::api::FMPClientTrait;
use crate::config;
use super::currencies::{convert_currency, get_rate_map_from_db};
use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

pub async fn fetch_historical_marketcaps(
    pool: &SqlitePool,
    start_year: i32,
    end_year: i32,
) -> Result<()> {
    let config = config::load_config()?;
    let tickers = [config.non_us_tickers, config.us_tickers].concat();

    // Get FMP client for market data
    let api_key = std::env::var("FINANCIALMODELINGPREP_API_KEY")
        .expect("FINANCIALMODELINGPREP_API_KEY must be set");
    let fmp_client = Arc::new(api::FMPClient::new(api_key));

    println!(
        "Fetching historical market caps from {} to {}",
        start_year, end_year
    );

    for year in start_year..=end_year {
        // Get Dec 31st of each year
        let date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
        let naive_dt = NaiveDateTime::new(date, NaiveTime::default());
        let datetime_utc = naive_dt.and_utc();
        println!("Fetching exchange rates for {}", naive_dt);
        let rate_map = get_rate_map_from_db(pool).await?;

        for ticker in &tickers {
            let market_cap = match fmp_client
                .get_historical_market_cap(ticker, &datetime_utc)
                .await?
            {
                Some(market_cap) => market_cap,
                None => {
                    eprintln!("No market cap data found for {} at {}", ticker, datetime_utc);
                    continue;
                }
            };

            // Get company details for additional info
            let details = match fmp_client.get_details(ticker, &rate_map).await? {
                Some(details) => details,
                None => {
                    eprintln!("No company details found for {}", ticker);
                    continue;
                }
            };

            // Convert currencies if needed
            let currency = details.currency_symbol.clone().unwrap_or_default();
            let market_cap_eur = convert_currency(
                market_cap,
                &currency,
                "EUR",
                &rate_map,
            );

            let market_cap_usd = convert_currency(
                market_cap,
                &currency,
                "USD",
                &rate_map,
            );

            // Store the Unix timestamp of the historical date
            let timestamp = naive_dt.and_utc().timestamp();

            // Extract values before the query
            let name = details.name.clone().unwrap_or_default();
            let exchange = details.extra.get("exchange")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let market_cap_value = details.market_cap.unwrap_or_default();

            // Insert into database
            sqlx::query!(
                r#"
                INSERT INTO market_caps (
                    ticker, name, market_cap_original, original_currency,
                    market_cap_eur, market_cap_usd, exchange, price,
                    active, timestamp
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                ticker,
                name,
                market_cap,
                currency,
                market_cap_eur,
                market_cap_usd,
                exchange,
                market_cap_value,
                true,
                timestamp,
            )
            .execute(pool)
            .await?;

            println!(
                "✅ Added historical market cap for {} on {}",
                ticker, naive_dt
            );
        }
    }

    Ok(())
}
