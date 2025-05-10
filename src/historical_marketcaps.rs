// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::api;
use crate::config;
use crate::currencies::{convert_currency, get_rate_map_from_db};
use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use tokio_postgres::Client; // Added
use std::sync::Arc;

pub async fn fetch_historical_marketcaps(
    client: &mut Client,
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
        let rate_map = get_rate_map_from_db(client).await?;

        for ticker in &tickers {
            match fmp_client
                .get_historical_market_cap(ticker, &datetime_utc)
                .await
            {
                Ok(market_cap) => {
                    // Convert currencies if needed
                    let market_cap_eur = convert_currency(
                        market_cap.market_cap_original,
                        &market_cap.original_currency,
                        "EUR",
                        &rate_map,
                    );

                    let market_cap_usd = convert_currency(
                        market_cap.market_cap_original,
                        &market_cap.original_currency,
                        "USD",
                        &rate_map,
                    );

                    // Store the Unix timestamp of the historical date
                    let timestamp = naive_dt.and_utc().timestamp();

                    // Insert into database
                    // Values from Ok(market_cap) match:
                    // ticker: &String (loop variable)
                    // market_cap.name: String
                    // market_cap.market_cap_original: f64
                    // market_cap.original_currency: String
                    // market_cap_eur: f64 (calculated)
                    // market_cap_usd: f64 (calculated)
                    // market_cap.exchange: String
                    // market_cap.price: f64
                    // active: bool (hardcoded as true previously)
                    // timestamp: i64 (api_timestamp_val)

                    client
                        .execute(
                            r#"
                            INSERT INTO market_caps (
                                ticker, name, market_cap_original, original_currency,
                                market_cap_eur, market_cap_usd, exchange, price,
                                active, api_timestamp, created_at, updated_at
                            )
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())
                            ON CONFLICT (ticker, api_timestamp) DO UPDATE SET
                                name = EXCLUDED.name,
                                market_cap_original = EXCLUDED.market_cap_original,
                                original_currency = EXCLUDED.original_currency,
                                market_cap_eur = EXCLUDED.market_cap_eur,
                                market_cap_usd = EXCLUDED.market_cap_usd,
                                exchange = EXCLUDED.exchange,
                                price = EXCLUDED.price,
                                active = EXCLUDED.active,
                                updated_at = NOW()
                            "#,
                            &[
                                &ticker,
                                &market_cap.name,
                                &(market_cap.market_cap_original as i64),
                                &market_cap.original_currency,
                                &(market_cap_eur as i64),
                                &(market_cap_usd as i64),
                                &market_cap.exchange,
                                &market_cap.price,
                                &true, // active
                                &timestamp, // api_timestamp
                            ],
                        )
                        .await?;

                    println!(
                        "✅ Added historical market cap for {} on {}",
                        ticker, naive_dt
                    );
                }
                Err(e) => {
                    eprintln!(
                        "❌ Failed to fetch market cap for {} on {}: {}",
                        ticker, naive_dt, e
                    );
                }
            }
        }
    }

    Ok(())
}
