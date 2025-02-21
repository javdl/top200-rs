use super::currencies::insert_forex_rate;
use crate::api::FMPClient;
use crate::api::FMPClientTrait;
use anyhow::Result;
use chrono::Local;
use csv::Writer;
use sqlx::sqlite::SqlitePool;
use std::fs;
use std::path::PathBuf;

/// Update exchange rates in the database
pub async fn update_exchange_rates(fmp_client: &FMPClient, pool: &SqlitePool) -> Result<()> {
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
    let timestamp = Local::now().timestamp();
    for rate in exchange_rates {
        if let (Some(name), Some(price)) = (rate.name.as_deref(), rate.price) {
            insert_forex_rate(pool, &name, price, price, timestamp).await?;
        }
    }

    println!("✅ Exchange rates updated in database");
    Ok(())
}

/// Export exchange rates to CSV
pub async fn export_exchange_rates_csv(fmp_client: &FMPClient, pool: &SqlitePool) -> Result<()> {
    // Create output directory if it doesn't exist
    let output_dir = PathBuf::from("output");
    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
    }

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

    // Create CSV file
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("output/exchange_rates_{}.csv", timestamp);
    let file = fs::File::create(&filename)?;
    let mut writer = Writer::from_writer(file);

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

    // Write rates and insert into database
    let timestamp = Local::now().timestamp();
    for rate in exchange_rates {
        if let (Some(name), Some(price)) = (rate.name.as_deref(), rate.price) {
            // Split the symbol into base and quote currencies (e.g., "EUR/USD" -> ["EUR", "USD"])
            let currencies: Vec<&str> = name.split('/').collect();
            let (base, quote) = if currencies.len() == 2 {
                (currencies[0], currencies[1])
            } else {
                ("", "")
            };

            // Write to CSV
            writer.write_record(&[
                &name,
                &price.to_string().as_str(),
                &rate
                    .changes_percentage
                    .map_or_else(|| "".to_string(), |v| v.to_string())
                    .as_str(),
                &rate
                    .change
                    .map_or_else(|| "".to_string(), |v| v.to_string())
                    .as_str(),
                &rate
                    .day_low
                    .map_or_else(|| "".to_string(), |v| v.to_string())
                    .as_str(),
                &rate
                    .day_high
                    .map_or_else(|| "".to_string(), |v| v.to_string())
                    .as_str(),
                base,
                quote,
                &timestamp.to_string().as_str(),
            ])?;

            // Insert into database
            insert_forex_rate(pool, &name, price, price, timestamp).await?;
        }
    }

    println!("✅ Exchange rates written to {}", filename);
    Ok(())
}
