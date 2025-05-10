// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

mod api;
mod config;
mod currencies;
mod db;
mod details_eu_fmp;
mod details_us_polygon;
mod exchange_rates;
mod historical_marketcaps;
mod marketcaps;
mod models;
mod monthly_historical_marketcaps;
mod ticker_details;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
// use sqlx::sqlite::SqlitePool; // Replaced by tokio_postgres::Client
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
    /// Fetch monthly historical market caps
    FetchMonthlyHistoricalMarketCaps { start_year: i32, end_year: i32 },
    /// Add a currency
    AddCurrency { code: String, name: String },
    /// List currencies
    ListCurrencies,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    // Establish PostgreSQL connection
    let mut client = db::connect().await.map_err(|e| {
        eprintln!("Database connection setup failed: {}", e);
        anyhow::anyhow!("Failed to connect to PostgreSQL. Caused by: {}", e)
    })?;

    // Apply database migrations
    db::run_migrations(&mut client).await.map_err(|e| {
        eprintln!("Database migration failed: {}", e);
        anyhow::anyhow!("Failed to run database migrations. Caused by: {}", e)
    })?;

    // The `client` variable (type tokio_postgres::Client) now replaces the old `pool`.
    // All functions below that used `&pool` will need to be refactored
    // to accept `&mut tokio_postgres::Client` or `&tokio_postgres::Client`
    // and use `tokio-postgres` APIs for their database operations.
    // This will cause compilation errors until those functions are updated.

    match cli.command {
        Some(Commands::ExportUs) => details_us_polygon::export_details_us_csv().await?,
        Some(Commands::ExportEu) => details_eu_fmp::export_details_eu_csv(&mut client).await?,
        Some(Commands::ExportCombined) => {
            marketcaps::marketcaps(&mut client).await?;
        }
        Some(Commands::ListUs) => details_us_polygon::list_details_us().await?,
        Some(Commands::ListEu) => details_eu_fmp::list_details_eu(&mut client).await?,
        Some(Commands::ExportRates) => {
            let api_key = env::var("FINANCIALMODELINGPREP_API_KEY")
                .expect("FINANCIALMODELINGPREP_API_KEY must be set");
            let fmp_client = api::FMPClient::new(api_key);
            exchange_rates::update_exchange_rates(&fmp_client, &mut client).await?;
        }
        Some(Commands::FetchHistoricalMarketCaps {
            start_year,
            end_year,
        }) => {
            historical_marketcaps::fetch_historical_marketcaps(&mut client, start_year, end_year)
                .await?;
        }
        Some(Commands::FetchMonthlyHistoricalMarketCaps {
            start_year,
            end_year,
        }) => {
            monthly_historical_marketcaps::fetch_monthly_historical_marketcaps(
                &mut client,
                start_year,
                end_year,
            )
            .await?;
        }
        Some(Commands::AddCurrency { code, name }) => {
            let api_key = env::var("FINANCIALMODELINGPREP_API_KEY")
                .expect("FINANCIALMODELINGPREP_API_KEY must be set");
            let fmp_client = api::FMPClient::new(api_key);
            currencies::update_currencies(&fmp_client, &mut client).await?;
            println!("✅ Currencies updated from FMP API");

            // Also add the manually specified currency
            currencies::insert_currency(&mut client, &code, &name).await?;
            println!("✅ Added currency: {} ({})", name, code);
        }
        Some(Commands::ListCurrencies) => {
            let currencies = currencies::list_currencies(&mut client).await?;
            for (code, name) in currencies {
                println!("{}: {}", code, name);
            }
        }
        None => {
            // Default command, presumably marketcaps
            marketcaps::marketcaps(&mut client).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
