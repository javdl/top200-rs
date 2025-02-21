// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

mod api;
mod config;
mod db;
mod details_eu_fmp;
mod details_us_polygon;
mod models;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use models::currencies;
use models::exchange_rates;
use models::historical_marketcaps;
use models::marketcaps;
use models::ticker_details;
// use sqlx::sqlite::SqlitePool;
use std::env;
use tokio;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Export US market caps to CSV
    ExportUs,
    /// Export EU market caps to CSV
    ExportEu,
    /// Export combined market caps to CSV
    ExportCombined,
    /// List US market caps
    ListUs,
    /// List EU market caps
    ListEu,
    /// Export exchange rates to CSV
    ExportRates,
    /// Fetch historical market caps
    FetchHistoricalMarketCaps { start_year: i32, end_year: i32 },
    /// Add a currency
    AddCurrency { code: String, name: String },
    /// List currencies
    ListCurrencies,
    /// Get market caps
    GetMarketCaps,
    /// Export market caps
    ExportMarketCaps,
    /// Export top 100 active companies
    ExportTop100Active,
    /// Get ticker details
    GetTickerDetails { ticker: String },
    /// List ticker details
    ListTickerDetails,
    /// Get forex rates
    GetForexRates {
        symbol: String,
        start_timestamp: i64,
        end_timestamp: i64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".to_string());
    let pool = db::create_db_pool(&db_url).await?;

    match cli.command {
        Some(Commands::ExportUs) => details_us_polygon::export_details_us_csv(&pool).await?,
        Some(Commands::ExportEu) => details_eu_fmp::export_details_eu_csv(&pool).await?,
        Some(Commands::ExportCombined) => {
            marketcaps::update_market_caps(&pool).await?;
        }
        Some(Commands::ListUs) => details_us_polygon::list_details_us(&pool).await?,
        Some(Commands::ListEu) => details_eu_fmp::list_details_eu(&pool).await?,
        Some(Commands::ExportRates) => {
            let api_key = env::var("FINANCIALMODELINGPREP_API_KEY")
                .expect("FINANCIALMODELINGPREP_API_KEY must be set");
            let fmp_client = api::FMPClient::new(api_key);
            exchange_rates::export_exchange_rates_csv(&fmp_client, &pool).await?;
        }
        Some(Commands::FetchHistoricalMarketCaps {
            start_year,
            end_year,
        }) => {
            historical_marketcaps::fetch_historical_marketcaps(&pool, start_year, end_year).await?;
        }
        Some(Commands::AddCurrency { code, name }) => {
            let api_key = env::var("FINANCIALMODELINGPREP_API_KEY")
                .expect("FINANCIALMODELINGPREP_API_KEY must be set");
            let fmp_client = api::FMPClient::new(api_key);
            currencies::update_currencies(&fmp_client, &pool).await?;
            println!("✅ Currencies updated from FMP API");

            // Also add the manually specified currency
            currencies::insert_currency(&pool, &code, &name).await?;
            println!("✅ Added currency: {} ({})", name, code);
        }
        Some(Commands::ListCurrencies) => {
            let currencies = currencies::list_currencies(&pool).await?;
            for (code, name) in currencies {
                println!("{}: {}", code, name);
            }
        }
        Some(Commands::GetMarketCaps) => {
            let market_caps = marketcaps::get_market_caps(&pool).await?;
            for (market_cap, details) in market_caps {
                println!("Market Cap: {}, Details: {:?}", market_cap, details);
            }
        }
        Some(Commands::ExportMarketCaps) => {
            marketcaps::export_market_caps(&pool).await?;
        }
        Some(Commands::ExportTop100Active) => {
            marketcaps::export_top_100_active(&pool).await?;
        }
        Some(Commands::GetTickerDetails { ticker }) => {
            if let Some(details) = ticker_details::get_ticker_details(&pool, &ticker).await? {
                println!("Details for {}: {:?}", ticker, details);
            } else {
                println!("No details found for ticker: {}", ticker);
            }
        }
        Some(Commands::ListTickerDetails) => {
            let details = ticker_details::list_ticker_details(&pool).await?;
            for detail in details {
                println!("{:?}", detail);
            }
        }
        Some(Commands::GetForexRates {
            symbol,
            start_timestamp,
            end_timestamp,
        }) => {
            let rates =
                currencies::get_forex_rates(&pool, &symbol, start_timestamp, end_timestamp).await?;
            for (ask, bid, timestamp) in rates {
                println!(
                    "Symbol: {}, Ask: {}, Bid: {}, Timestamp: {}",
                    symbol, ask, bid, timestamp
                );
            }
        }
        None => {
            marketcaps::marketcaps(&pool).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
