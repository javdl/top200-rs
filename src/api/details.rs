use anyhow::Result;
use std::collections::HashMap;
use std::env;

use crate::models::Details;
use crate::models::exchange_rates::RateMap;
use super::fmp_client::FMPClient;

pub async fn get_details_eu(ticker: &str, rate_map: &RateMap) -> Result<HashMap<String, String>, String> {
    // This is a simplified wrapper - consider updating to match the implementation in src/api.rs
    Ok(HashMap::new())
}

// Add the full implementation from src/api.rs
pub async fn get_details_eu_full(ticker: &str, rate_map: &HashMap<String, f64>) -> Result<Details> {
    let api_key = env::var("FINANCIALMODELINGPREP_API_KEY")
        .expect("FINANCIALMODELINGPREP_API_KEY must be set");
    let client = FMPClient::new(api_key);
    client
        .get_details(ticker, rate_map)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No details found for ticker {}", ticker))
} 