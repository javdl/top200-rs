// SPDX-FileCopyrightText: 2025 Joost van der Laan
// SPDX-License-Identifier: AGPL-3.0-only

use crate::api::FMPClient;
use crate::currencies::insert_forex_rate;
use anyhow::Result;
use chrono::Local;
use tokio_postgres::Client;

/// Update exchange rates in the database
pub async fn update_exchange_rates(fmp_client: &FMPClient, client: &mut Client) -> Result<()> {
    // Fetch exchange rates
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

    // Store rates in database
    let current_api_timestamp = Local::now().timestamp(); // i64, used as the api_timestamp for these fetched rates
    for rate in exchange_rates {
        // Assuming FMPClient::ExchangeRate has name: Option<String> (symbol) and price: Option<f64> (rate)
        if let (Some(symbol_name), Some(price)) = (rate.name, rate.price) {
            // insert_forex_rate expects: client, symbol, ask, bid, api_timestamp_val
            // Here, we use 'price' for both ask and bid as per previous logic.
            insert_forex_rate(client, &symbol_name, price, price, current_api_timestamp).await?;
        }
    }

    println!("✅ Exchange rates updated in database");
    Ok(())
}
