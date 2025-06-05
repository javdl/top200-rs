// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use crate::{api, config};

#[derive(Debug)]
pub struct TickerDetails {
    pub ticker: String,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
    pub employees: Option<String>,
}

/// Update ticker details in the database
pub async fn update_ticker_details(pool: &SqlitePool, details: &TickerDetails) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO ticker_details (ticker, description, homepage_url, employees)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(ticker) DO UPDATE SET
            description = excluded.description,
            homepage_url = excluded.homepage_url,
            employees = excluded.employees,
            updated_at = CURRENT_TIMESTAMP
        "#,
        details.ticker,
        details.description,
        details.homepage_url,
        details.employees,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch and store details for all configured tickers
pub async fn fetch_and_store_details(pool: &SqlitePool) -> Result<()> {
    let config = config::load_config()?;
    let tickers = [config.non_us_tickers, config.us_tickers].concat();
    
    let api_key = std::env::var("FINANCIALMODELINGPREP_API_KEY")
        .expect("FINANCIALMODELINGPREP_API_KEY must be set");
    let fmp_client = api::FMPClient::new(api_key);
    
    println!("Fetching details for {} tickers...", tickers.len());
    
    for ticker in &tickers {
        match fmp_client.get_details(ticker, &std::collections::HashMap::new()).await {
            Ok(details) => {
                let ticker_details = TickerDetails {
                    ticker: details.ticker.clone(),
                    description: details.description,
                    homepage_url: details.homepage_url,
                    employees: details.employees,
                };
                
                if let Err(e) = update_ticker_details(pool, &ticker_details).await {
                    eprintln!("Failed to store details for {}: {}", ticker, e);
                } else {
                    println!("✅ Updated details for {}", ticker);
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch details for {}: {}", ticker, e);
            }
        }
    }
    
    println!("✅ Finished fetching ticker details");
    Ok(())
}
