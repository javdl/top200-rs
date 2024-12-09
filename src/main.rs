mod api;
mod models;
mod tui;
mod viz;
mod config;
mod utils;

use std::{collections::HashMap, env, path::PathBuf, time::Duration};
use anyhow::Result;
use chrono::{Local, NaiveDate};
use csv::Writer;
use dotenv::dotenv;
use tokio;

pub use utils::convert_currency;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let options = vec![
        "Export combined US & non-US stock marketcaps to CSV & generate treemap".to_string(),
        "Export currency exchange rates to CSV".to_string(),
        "List US stock marketcaps (Polygon API)".to_string(),
        "List EU stock marketcaps".to_string(),
        "Export US stock marketcaps to CSV".to_string(),
        "Export EU stock marketcaps to CSV".to_string(),
        "Generate Market Heatmap".to_string(),
        "Exit".to_string(),
    ];

    let selected = tui::start_tui(options)?;

    match selected {
        Some(ans) => match ans.as_str() {
            "Export combined US & non-US stock marketcaps to CSV & generate treemap" => {
                let api_key = env::var("FINANCIALMODELINGPREP_API_KEY").expect("FINANCIALMODELINGPREP_API_KEY must be set");
                let fmp_client = api::FMPClient::new(api_key);
                export_details_combined_csv(&fmp_client).await?;
            }
            "Export currency exchange rates to CSV" => {
                let api_key = env::var("FINANCIALMODELINGPREP_API_KEY").expect("FINANCIALMODELINGPREP_API_KEY must be set");
                let fmp_client = api::FMPClient::new(api_key);
                export_exchange_rates_csv(&fmp_client).await?;
            }
            "List US stock marketcaps (Polygon API)" => list_details_us().await?,
            "List EU stock marketcaps" => list_details_eu().await?,
            "Export US stock marketcaps to CSV" => export_details_us_csv().await?,
            "Export EU stock marketcaps to CSV" => export_details_eu_csv().await?,
            "Generate Market Heatmap" => export_details_combined_csv(&api::FMPClient::new(env::var("FINANCIALMODELINGPREP_API_KEY").expect("FINANCIALMODELINGPREP_API_KEY must be set"))).await?,
            "Exit" => println!("Exiting..."),
            _ => unreachable!(),
        },
        None => println!("Exiting..."),
    }

    Ok(())
}

async fn export_details_eu_csv() -> Result<()> {
    let config = config::load_config()?;
    let tickers = config.non_us_tickers;

    // Create output directory if it doesn't exist
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;

    // Create CSV file with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let csv_path = output_dir.join(format!("eu_marketcaps_{}.csv", timestamp));
    let mut writer = Writer::from_path(&csv_path)?;

    // Write header
    writer.write_record(&[
        "Ticker",
        "Company Name",
        "Market Cap",
        "Currency",
        "Exchange",
        "Price",
        "Active",
        "Description",
        "Homepage URL",
        "Employees",
        "Revenue",
        "Revenue (USD)",
        "Working Capital Ratio",
        "Quick Ratio",
        "EPS",
        "P/E Ratio",
        "D/E Ratio",
        "ROE",
    ])?;

    let rate_map = get_rate_map();

    let mut tasks = Vec::new();

    for ticker in tickers {
        let ticker = ticker.clone();
        let rate_map = rate_map.clone();
        tasks.push(tokio::spawn(async move {
            let details = api::get_details_eu(&ticker, &rate_map).await;
            (ticker, details)
        }));
    }

    for task in tasks {
        let (ticker, details) = task.await?;
        match details {
            Ok(details) => {
                writer.write_record(&[
                    &details.ticker,
                    &details.name.unwrap_or_default(),
                    &details.market_cap.map(|m| m.to_string()).unwrap_or_default(),
                    &details.currency_symbol.unwrap_or_default(),
                    &details.extra.get("exchange").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    &details.extra.get("price").map(|v| v.to_string()).unwrap_or_default(),
                    &details.active.map(|a| a.to_string()).unwrap_or_default(),
                    &details.description.unwrap_or_default(),
                    &details.homepage_url.unwrap_or_default(),
                    &details.employees.unwrap_or_default(),
                    &details.revenue.map(|r| r.to_string()).unwrap_or_default(),
                    &details.revenue_usd.map(|r| r.to_string()).unwrap_or_default(),
                    &details.working_capital_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.quick_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.eps.map(|r| r.to_string()).unwrap_or_default(),
                    &details.pe_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.debt_equity_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.roe.map(|r| r.to_string()).unwrap_or_default(),
                ])?;
                println!("✅ Data written to CSV");
            }
            Err(e) => {
                eprintln!("Error fetching details for {}: {}", ticker, e);
                // Write empty row for failed ticker
                let error_msg = format!("Error: {}", e);
                writer.write_record(&[
                    &ticker,
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    &error_msg,
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                ])?;
            }
        }
    }

    writer.flush()?;
    println!("\n✅ CSV file created at: {}", csv_path.display());

    Ok(())
}

async fn export_details_us_csv() -> Result<()> {
    let config = config::load_config()?;
    let tickers = config.us_tickers;
    let api_key = env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY must be set");
    let client = api::PolygonClient::new(api_key);
    let date = NaiveDate::from_ymd_opt(2023, 11, 1).unwrap();

    // Create output directory if it doesn't exist
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;

    // Create CSV file with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let csv_path = output_dir.join(format!("us_marketcaps_{}.csv", timestamp));
    let mut writer = Writer::from_path(&csv_path)?;

    // Write header
    writer.write_record(&[
        "Ticker",
        "Company Name",
        "Market Cap",
        "Currency",
        "Active",
        "Description",
        "Homepage URL",
        "Employees",
        "Revenue",
        "Revenue (USD)",
        "Working Capital Ratio",
        "Quick Ratio",
        "EPS",
        "P/E Ratio",
        "D/E Ratio",
        "ROE",
    ])?;

    for (i, ticker) in tickers.iter().enumerate() {
        println!("\nFetching the marketcap for {} ({}/{}) ⌛️", ticker, i + 1, tickers.len());
        match client.get_details(ticker, date).await {
            Ok(details) => {
                writer.write_record(&[
                    &details.ticker,
                    &details.name.unwrap_or_default(),
                    &details.market_cap.map(|m| m.to_string()).unwrap_or_default(),
                    &details.currency_symbol.unwrap_or_default(),
                    &details.active.map(|a| a.to_string()).unwrap_or_default(),
                    &details.description.unwrap_or_default(),
                    &details.homepage_url.unwrap_or_default(),
                    &details.employees.unwrap_or_default(),
                    &details.revenue.map(|r| r.to_string()).unwrap_or_default(),
                    &details.revenue_usd.map(|r| r.to_string()).unwrap_or_default(),
                    &details.working_capital_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.quick_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.eps.map(|r| r.to_string()).unwrap_or_default(),
                    &details.pe_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.debt_equity_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.roe.map(|r| r.to_string()).unwrap_or_default(),
                ])?;
                println!("✅ Data written to CSV");
            }
            Err(e) => {
                eprintln!("Error fetching details for {}: {}", ticker, e);
                // Write empty row for failed ticker
                let error_msg = format!("Error: {}", e);
                writer.write_record(&[
                    &ticker,
                    "",
                    "",
                    "",
                    "",
                    &error_msg,
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                ])?;
            }
        }
    }

    writer.flush()?;
    println!("\n✅ CSV file created at: {}", csv_path.display());

    Ok(())
}

async fn list_details_us() -> Result<()> {
    let config = config::load_config()?;
    let tickers = config.us_tickers;
    let api_key = env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY must be set");
    let client = api::PolygonClient::new(api_key);
    let date = NaiveDate::from_ymd_opt(2023, 11, 1).unwrap();

    for (i, ticker) in tickers.iter().enumerate() {
        println!("\nFetching the marketcap for {} ({}/{}) ⌛️", ticker, i + 1, tickers.len());
        match client.get_details(ticker, date).await {
            Ok(details) => {
                println!("Company: {}", details.name.unwrap_or_default());
                if let Some(market_cap) = details.market_cap {
                    println!("Market Cap: {} {}", details.currency_symbol.unwrap_or_default(), market_cap);
                }
                println!("Active: {}", details.active.unwrap_or_default());
                println!("---");
            }
            Err(e) => eprintln!("Error fetching details for {}: {}", ticker, e),
        }
    }

    Ok(())
}

async fn list_details_eu() -> Result<()> {
    let config = config::load_config()?;
    let tickers = config.non_us_tickers;

    let rate_map = get_rate_map();

    let mut tasks = Vec::new();

    for ticker in tickers {
        let ticker = ticker.clone();
        let rate_map = rate_map.clone();
        tasks.push(tokio::spawn(async move {
            let details = api::get_details_eu(&ticker, &rate_map).await;
            (ticker, details)
        }));
    }

    for task in tasks {
        let (ticker, details) = task.await?;
        match details {
            Ok(details) => {
                println!("Company: {}", details.name.unwrap_or_default());
                if let Some(market_cap) = details.market_cap {
                    println!("Market Cap: {} {}", details.currency_symbol.unwrap_or_default(), market_cap);
                }
                println!("Active: {}", details.active.unwrap_or_default());
                println!("---");
            }
            Err(e) => eprintln!("Error fetching details for {}: {}", ticker, e),
        }
    }

    Ok(())
}

async fn export_details_combined_csv(fmp_client: &api::FMPClient) -> Result<()> {
    let config = config::load_config()?;
    let tickers = [config.non_us_tickers, config.us_tickers].concat();
    
    // First fetch exchange rates
    println!("Fetching current exchange rates...");
    let exchange_rates = match fmp_client.get_exchange_rates().await {
        Ok(rates) => {
            println!("✅ Exchange rates fetched");
            rates
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to fetch exchange rates: {}", e));
        }
    };

    // Create a map of currency pairs to rates
    let mut rate_map: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for rate in exchange_rates {
        rate_map.insert(rate.name.clone(), rate.price);
    }

    // Helper function to convert currency symbol to code and get divisor
    let currency_to_code = |currency: &str| -> (String, f64) {
        match currency {
            "€" => ("EUR".to_string(), 1.0),
            "$" => ("USD".to_string(), 1.0),
            "£" => ("GBP".to_string(), 1.0),
            "¥" => ("JPY".to_string(), 1.0),
            "₣" => ("CHF".to_string(), 1.0),
            "kr" => ("SEK".to_string(), 1.0),
            "GBp" => ("GBP".to_string(), 100.0),  // Convert pence to pounds
            "GBX" => ("GBP".to_string(), 100.0),  // Alternative notation for pence
            _ => (currency.to_string(), 1.0)
        }
    };

    // Helper function to convert amount from source currency to target currency
    let convert_currency = |amount: f64, from_currency: &str, to_currency: &str| -> f64 {
        let (from_code, from_divisor) = currency_to_code(from_currency);
        let amount = amount / from_divisor; // Convert to main currency unit if needed
        
        if from_code == to_currency {
            amount
        } else {
            // First try direct conversion
            let pair = format!("{}/{}", from_code, to_currency);
            if let Some(&rate) = rate_map.get(&pair) {
                amount * rate
            } else {
                // Try inverse conversion
                let inverse_pair = format!("{}/{}", to_currency, from_code);
                if let Some(&rate) = rate_map.get(&inverse_pair) {
                    amount / rate
                } else {
                    // If no direct conversion exists, try through USD
                    let to_usd = format!("{}/USD", from_code);
                    let usd_to_target = rate_map.get(&format!("USD/{}", to_currency)).copied().unwrap_or_else(|| {
                        if to_currency == "EUR" {
                            0.92 // Fallback USD to EUR
                        } else {
                            1.0 // Fallback for USD
                        }
                    });
                    
                    if let Some(&rate) = rate_map.get(&to_usd) {
                        amount * rate * usd_to_target
                    } else {
                        println!("⚠️  Warning: No conversion rate found for {} to {}", from_currency, to_currency);
                        amount // Return unconverted amount as fallback
                    }
                }
            }
        }
    };

    // Convert exchange prefixes to FMP format
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("output/combined_marketcaps_{}.csv", timestamp);
    let file = std::fs::File::create(&filename)?;
    let mut writer = csv::Writer::from_writer(file);

    // Write headers
    writer.write_record(&[
        "Ticker",
        "Name",
        "Market Cap (Original)",
        "Original Currency",
        "Market Cap (EUR)",
        "Market Cap (USD)",
        "Exchange",
        "Price",
        "Active",
        "Description",
        "Homepage URL",
        "Employees",
        "Revenue",
        "Revenue (USD)",
        "Working Capital Ratio",
        "Quick Ratio",
        "EPS",
        "P/E Ratio",
        "D/E Ratio",
        "ROE",
    ])?;

    // Collect all results first
    let mut results = Vec::new();
    for ticker in tickers {
        println!("Fetching data for {}", ticker);
        match fmp_client.get_details(&ticker, &rate_map).await {
            Ok(details) => {
                let original_market_cap = details.market_cap.unwrap_or(0.0);
                let currency = details.currency_symbol.clone().unwrap_or_default();
                let eur_market_cap = convert_currency(original_market_cap, &currency, "EUR");
                let usd_market_cap = convert_currency(original_market_cap, &currency, "USD");
                
                results.push((
                    eur_market_cap,
                    vec![
                        details.ticker,
                        details.name.unwrap_or_default(),
                        original_market_cap.round().to_string(),
                        currency,
                        eur_market_cap.round().to_string(),
                        usd_market_cap.round().to_string(),
                        details.extra.get("exchange").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        details.extra.get("price").map(|v| v.to_string()).unwrap_or_default(),
                        details.active.map(|a| a.to_string()).unwrap_or_default(),
                        details.description.unwrap_or_default(),
                        details.homepage_url.unwrap_or_default(),
                        details.employees.unwrap_or_default(),
                        details.revenue.map(|r| r.to_string()).unwrap_or_default(),
                        details.revenue_usd.map(|r| r.to_string()).unwrap_or_default(),
                        details.working_capital_ratio.map(|r| r.to_string()).unwrap_or_default(),
                        details.quick_ratio.map(|r| r.to_string()).unwrap_or_default(),
                        details.eps.map(|r| r.to_string()).unwrap_or_default(),
                        details.pe_ratio.map(|r| r.to_string()).unwrap_or_default(),
                        details.debt_equity_ratio.map(|r| r.to_string()).unwrap_or_default(),
                        details.roe.map(|r| r.to_string()).unwrap_or_default(),
                    ]
                ));
                println!("✅ Data collected");
            }
            Err(e) => {
                eprintln!("Error fetching data for {}: {}", ticker, e);
                results.push((
                    0.0,
                    vec![
                        ticker.to_string(),
                        "ERROR".to_string(),
                        "0".to_string(),
                        "".to_string(),
                        "0".to_string(),
                        "0".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        format!("Error: {}", e),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ]
                ));
            }
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // Sort by EUR market cap (highest to lowest)
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Write sorted results to CSV
    for (_, record) in &results {  
        writer.write_record(&*record)?;  // Dereference to get &[String]
    }

    println!("📝 CSV file created: {}", filename);
    println!("💶 Results are sorted by market cap in EUR (highest to lowest)");

    // Generate heatmap
    println!("\nGenerating market heatmap...");
    let heatmap_filename = format!("output/heatmap_{}.png", timestamp);
    generate_market_heatmap(&results, &heatmap_filename)?;
    println!("🎨 Heatmap generated: {}", heatmap_filename);

    Ok(())
}

fn generate_market_heatmap(results: &[(f64, Vec<String>)], output_path: &str) -> Result<()> {
    let mut stocks = Vec::new();
    
    // Only take the first 100 results since they're already sorted by market cap
    for (market_cap, data) in results.iter().take(100) {
        if *market_cap > 0.0 {  // Skip error entries
            stocks.push((*market_cap, viz::StockData {
                symbol: data[0].clone(),
                market_cap_eur: *market_cap,
                employees: data[11].clone(),  
            }));
        }
    }

    println!("📊 Generating heatmap with top {} companies", stocks.len());
    viz::create_market_heatmap(stocks.into_iter().map(|(_, data)| data).collect(), output_path)?;
    Ok(())
}

async fn export_exchange_rates_csv(fmp_client: &api::FMPClient) -> Result<()> {
    println!("Fetching current exchange rates...");
    
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("output/exchange_rates_{}.csv", timestamp);
    let file = std::fs::File::create(&filename)?;
    let mut writer = csv::Writer::from_writer(file);

    // Write headers
    writer.write_record(&[
        "Symbol",
        "Rate",
        "Change %",
        "Change",
        "Day Low",
        "Day High",
        "Base Currency",
        "Quote Currency",
        "Timestamp",
    ])?;

    match fmp_client.get_exchange_rates().await {
        Ok(rates) => {
            for rate in rates {
                // Split the symbol into base and quote currencies (e.g., "EUR/USD" -> ["EUR", "USD"])
                let currencies: Vec<&str> = rate.name.split('/').collect();
                let (base, quote) = if currencies.len() == 2 {
                    (currencies[0], currencies[1])
                } else {
                    ("", "")
                };

                writer.write_record(&[
                    &rate.name,
                    &rate.price.to_string(),
                    &rate.changes_percentage.map_or_else(|| "".to_string(), |v| v.to_string()),
                    &rate.change.map_or_else(|| "".to_string(), |v| v.to_string()),
                    &rate.day_low.map_or_else(|| "".to_string(), |v| v.to_string()),
                    &rate.day_high.map_or_else(|| "".to_string(), |v| v.to_string()),
                    base,
                    quote,
                    &rate.timestamp.to_string(),
                ])?;
            }
            println!("✅ Exchange rates written to CSV");
        }
        Err(e) => {
            eprintln!("Error fetching exchange rates: {}", e);
            writer.write_record(&[
                "ERROR",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                &format!("Error: {}", e),
            ])?;
        }
    }

    println!("📝 CSV file created: {}", filename);
    Ok(())
}

async fn export_marketcap_with_progress(tickers: Vec<String>, output_path: &str) -> Result<()> {
    let mut writer = Writer::from_path(output_path)?;

    writer.write_record(&[
        "Ticker",
        "Market Cap",
        "Name",
        "Currency Name",
        "Currency Symbol",
        "Active",
        "Description",
        "Homepage URL",
        "Employees",
        "Revenue",
        "Revenue (USD)",
        "Working Capital Ratio",
        "Quick Ratio",
        "EPS",
        "P/E Ratio",
        "D/E Ratio",
        "ROE",
    ])?;

    let rate_map = get_rate_map();
    let fmp_client = api::FMPClient::new(env::var("FINANCIALMODELINGPREP_API_KEY").expect("FINANCIALMODELINGPREP_API_KEY must be set"));

    for ticker in tickers {
        match fmp_client.get_details(&ticker, &rate_map).await {
            Ok(details) => {
                writer.write_record(&[
                    &details.ticker,
                    &details.market_cap.map(|m| m.to_string()).unwrap_or_default(),
                    &details.name.unwrap_or_default(),
                    &details.currency_name.unwrap_or_default(),
                    &details.currency_symbol.unwrap_or_default(),
                    &details.active.map(|a| a.to_string()).unwrap_or_default(),
                    &details.description.unwrap_or_default(),
                    &details.homepage_url.unwrap_or_default(),
                    &details.employees.unwrap_or_default(),
                    &details.revenue.map(|r| r.to_string()).unwrap_or_default(),
                    &details.revenue_usd.map(|r| r.to_string()).unwrap_or_default(),
                    &details.working_capital_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.quick_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.eps.map(|r| r.to_string()).unwrap_or_default(),
                    &details.pe_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.debt_equity_ratio.map(|r| r.to_string()).unwrap_or_default(),
                    &details.roe.map(|r| r.to_string()).unwrap_or_default(),
                ])?;
                println!("✅ Data written to CSV");
            }
            Err(e) => {
                eprintln!("Error fetching details for {}: {}", ticker, e);
                // Write empty row for failed ticker
                let error_msg = format!("Error: {}", e);
                writer.write_record(&[
                    &ticker,
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    &error_msg,
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                ])?;
            }
        }
    }

    writer.flush()?;
    println!("\n✅ CSV file created at: {}", output_path);
    Ok(())
}

async fn export_marketcap_to_json(tickers: Vec<String>, output_path: &str) -> Result<()> {
    let rate_map = get_rate_map();
    let fmp_client = api::FMPClient::new(env::var("FINANCIALMODELINGPREP_API_KEY").expect("FINANCIALMODELINGPREP_API_KEY must be set"));
    let mut stocks = Vec::new();

    for ticker in tickers {
        if let Ok(details) = fmp_client.get_details(&ticker, &rate_map).await {
            stocks.push(models::Stock {
                ticker: details.ticker,
                name: details.name.unwrap_or_default(),
                market_cap: details.market_cap.unwrap_or_default(),
                currency_name: details.currency_name.unwrap_or_default(),
                currency_symbol: details.currency_symbol.unwrap_or_default(),
                active: details.active.unwrap_or_default(),
                description: details.description.unwrap_or_default(),
                homepage_url: details.homepage_url.unwrap_or_default(),
                employees: details.employees.unwrap_or_default(),
                revenue: details.revenue.unwrap_or_default(),
                revenue_usd: details.revenue_usd.unwrap_or_default(),
                working_capital_ratio: details.working_capital_ratio.unwrap_or_default(),
                quick_ratio: details.quick_ratio.unwrap_or_default(),
                eps: details.eps.unwrap_or_default(),
                pe_ratio: details.pe_ratio.unwrap_or_default(),
                debt_equity_ratio: details.debt_equity_ratio.unwrap_or_default(),
                roe: details.roe.unwrap_or_default(),
            });
        } else {
            eprintln!("Error fetching details for {}", ticker);
        }
    }

    let json = serde_json::to_string_pretty(&stocks)?;
    std::fs::write(output_path, json)?;
    println!("✅ JSON file created at: {}", output_path);
    Ok(())
}

fn get_rate_map() -> HashMap<String, f64> {
    let mut rate_map = HashMap::new();
    
    // Base rates (currency to USD)
    rate_map.insert("EUR/USD".to_string(), 1.08);
    rate_map.insert("GBP/USD".to_string(), 1.25);
    rate_map.insert("CHF/USD".to_string(), 1.14);
    rate_map.insert("SEK/USD".to_string(), 0.096);
    rate_map.insert("DKK/USD".to_string(), 0.145);
    rate_map.insert("NOK/USD".to_string(), 0.093);
    rate_map.insert("JPY/USD".to_string(), 0.0068);
    rate_map.insert("HKD/USD".to_string(), 0.128);
    rate_map.insert("CNY/USD".to_string(), 0.139);
    rate_map.insert("BRL/USD".to_string(), 0.203);
    rate_map.insert("CAD/USD".to_string(), 0.737);
    
    rate_map
}
