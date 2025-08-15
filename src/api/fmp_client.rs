use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::{env, time::Duration};
use tokio::sync::Semaphore;
use tokio::time::sleep;

use crate::models::currencies::convert_currency;
use crate::models::{Details, FMPCompanyProfile, FMPIncomeStatement, FMPRatios};

#[async_trait::async_trait]
pub trait FMPClientTrait {
    async fn get_details(
        &self,
        ticker: &str,
        rate_map: &HashMap<String, f64>,
    ) -> Result<Option<Details>>;
    async fn get_historical_market_cap(
        &self,
        ticker: &str,
        date: &DateTime<Utc>,
    ) -> Result<Option<f64>>;
    async fn get_ratios(&self, ticker: &str) -> Result<Option<FMPRatios>>;
    async fn get_income_statement(&self, ticker: &str) -> Result<Option<FMPIncomeStatement>>;
    async fn get_exchange_rates(&self) -> Result<Vec<ExchangeRate>>;
}

#[derive(Clone)]
pub struct FMPClient {
    client: Client,
    pub api_key: String,
    rate_limiter: Arc<Semaphore>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ExchangeRate {
    pub name: Option<String>,
    pub price: Option<f64>,
    #[serde(rename = "changesPercentage")]
    pub changes_percentage: Option<f64>,
    pub change: Option<f64>,
    #[serde(rename = "dayLow")]
    pub day_low: Option<f64>,
    #[serde(rename = "dayHigh")]
    pub day_high: Option<f64>,
    #[serde(rename = "yearHigh")]
    pub year_high: Option<f64>,
    #[serde(rename = "yearLow")]
    pub year_low: Option<f64>,
    #[serde(rename = "marketCap")]
    pub market_cap: Option<f64>,
    #[serde(rename = "priceAvg50")]
    pub price_avg_50: Option<f64>,
    #[serde(rename = "priceAvg200")]
    pub price_avg_200: Option<f64>,
    pub volume: Option<f64>,
    #[serde(rename = "avgVolume")]
    pub avg_volume: Option<f64>,
    pub exchange: Option<String>,
    pub open: Option<f64>,
    #[serde(rename = "previousClose")]
    pub previous_close: Option<f64>,
    pub timestamp: i64,
}

#[async_trait::async_trait]
impl FMPClientTrait for FMPClient {
    async fn get_details(
        &self,
        ticker: &str,
        rate_map: &HashMap<String, f64>,
    ) -> Result<Option<Details>> {
        // Implement from src/api.rs
        if ticker.is_empty() {
            anyhow::bail!("ticker empty");
        }

        // ... rest of implementation from src/api.rs
        Ok(None) // Replace with actual implementation
    }

    async fn get_historical_market_cap(
        &self,
        ticker: &str,
        date: &DateTime<Utc>,
    ) -> Result<Option<f64>> {
        // Implement from src/api.rs
        Ok(None) // Replace with actual implementation
    }

    async fn get_ratios(&self, ticker: &str) -> Result<Option<FMPRatios>> {
        // Implement from src/api.rs
        Ok(None) // Replace with actual implementation
    }

    async fn get_income_statement(&self, ticker: &str) -> Result<Option<FMPIncomeStatement>> {
        // Implement from src/api.rs
        Ok(None) // Replace with actual implementation
    }

    async fn get_exchange_rates(&self) -> Result<Vec<ExchangeRate>> {
        // Implement from src/api.rs
        Ok(Vec::new()) // Replace with actual implementation
    }
}

impl FMPClient {
    pub fn new(api_key: String) -> Self {
        // Allow up to 300 concurrent requests per minute
        let rate_limiter = Arc::new(Semaphore::new(300));

        Self {
            client: Client::new(),
            api_key,
            rate_limiter,
        }
    }

    async fn make_request<T: for<'de> Deserialize<'de>>(&self, url: String) -> Result<T> {
        // Implement from src/api.rs
        Err(anyhow::anyhow!("Not implemented")) // Replace with actual implementation
    }
} 