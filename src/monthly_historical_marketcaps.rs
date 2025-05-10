// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::api;
use crate::config;
use crate::currencies::{convert_currency, get_rate_map_from_db};
use anyhow::Result;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::sync::Arc;
use tokio_postgres::Client; // Changed from sqlx::sqlite::SqlitePool

/// Fetches historical market caps for the last day of each month within the specified year range
pub async fn fetch_monthly_historical_marketcaps(
    client: &mut Client, // Changed
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
        "Fetching monthly historical market caps from {} to {}",
        start_year, end_year
    );

    for year in start_year..=end_year {
        for month in 1..=12 {
            // Skip future months in the current year
            if year == Utc::now().year() && month > Utc::now().month() {
                println!("Skipping future month: {}-{}", year, month);
                break;
            }

            // Get the last day of the month at 23:59:00 for consistency
            let last_day = get_last_day_of_month(year, month);
            let time = NaiveTime::from_hms_opt(23, 59, 0).unwrap(); // Use a fixed time
            let naive_dt = NaiveDateTime::new(last_day, time);
            let datetime_utc = naive_dt.and_utc();

            println!("Fetching exchange rates for end of {}-{}", year, month);
            let rate_map = get_rate_map_from_db(client).await?; // Changed

            for ticker in &tickers {
                println!(
                    "Fetching monthly historical for {} on {}...",
                    ticker, last_day
                );
                match fmp_client
                    .get_historical_market_cap(ticker, &datetime_utc)
                    .await
                {
                    Ok(historical_data) => {
                        let market_cap_original = historical_data.market_cap_original;
                        let original_currency = &historical_data.original_currency;

                        let market_cap_eur = convert_currency(
                            market_cap_original,
                            original_currency,
                            "EUR",
                            &rate_map,
                        );

                        let market_cap_usd = convert_currency(
                            market_cap_original,
                            original_currency,
                            "USD",
                            &rate_map,
                        );

                        let api_timestamp_val = datetime_utc.timestamp();

                        let name_val = &historical_data.name;
                        let exchange_val = &historical_data.exchange;
                        let price_val = historical_data.price;
                        let active_val = true; // HistoricalMarketCap doesn't have 'active', defaulting to true.

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
                                    &name_val,
                                    &(market_cap_original as i64),
                                    &original_currency,
                                    &(market_cap_eur as i64),
                                    &(market_cap_usd as i64),
                                    &exchange_val,
                                    &price_val,
                                    &active_val,
                                    &api_timestamp_val,
                                ],
                            )
                            .await?;

                        println!(
                            "✅ Added monthly historical market cap for {} on {}",
                            ticker,
                            naive_dt.date()
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "❌ Failed to fetch monthly market cap for {} on {}: {}",
                            ticker,
                            naive_dt.date(),
                            e
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

/// Helper function to get the last day of a given month
fn get_last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let first_day_next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };
    first_day_next_month.pred_opt().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_last_day_of_month() {
        assert_eq!(
            get_last_day_of_month(2025, 1),
            NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()
        );
        assert_eq!(
            get_last_day_of_month(2025, 2),
            NaiveDate::from_ymd_opt(2025, 2, 28).unwrap()
        );
        assert_eq!(
            get_last_day_of_month(2024, 2), // Leap year
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );
        assert_eq!(
            get_last_day_of_month(2025, 4),
            NaiveDate::from_ymd_opt(2025, 4, 30).unwrap()
        );
        assert_eq!(
            get_last_day_of_month(2025, 12),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
    }
}
