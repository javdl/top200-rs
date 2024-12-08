use anyhow::{Context, Result};
use chrono::NaiveDate;
use reqwest::Client;
use serde::Deserialize;
use serde_json;
use std::{env, time::Duration};
use tokio::time::sleep;

use crate::models::{Details, PolygonResponse, FMPCompanyProfile};

pub struct PolygonClient {
    client: Client,
    api_key: String,
}

pub struct FMPClient {
    client: Client,
    api_key: String,
}

impl FMPClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn get_details(&self, ticker: &str) -> Result<Details> {
        if ticker.is_empty() {
            anyhow::bail!("ticker empty");
        }

        // Add a small delay to stay within 300 calls/min limit
        sleep(Duration::from_millis(200)).await;

        let url = format!(
            "https://financialmodelingprep.com/api/v3/profile/{}?apikey={}",
            ticker,
            self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to get response text")?;

        if !status.is_success() {
            anyhow::bail!("API error: {} - {}", status, text);
        }

        // FMP returns an array with a single company profile
        let profiles: Vec<FMPCompanyProfile> = match serde_json::from_str(&text) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to parse response: {}", e);
                eprintln!("Raw response: {}", text);
                return Err(e).context("Failed to parse FMP response");
            }
        };
        
        let profile = profiles.first()
            .ok_or_else(|| anyhow::anyhow!("No company profile found"))?;

        // Map currency code to symbol
        let currency_symbol = match profile.currency.as_str() {
            "USD" => "$",
            "EUR" => "€",
            "GBP" => "£",
            "CHF" => "CHF",
            "DKK" => "kr",
            _ => &profile.currency,
        };

        // Parse shares outstanding from string
        let shares_outstanding = profile.shares_outstanding.replace(',', "").parse::<f64>().unwrap_or(0.0);

        Ok(Details {
            ticker: profile.symbol.clone(),
            market_cap: Some(profile.market_cap),
            name: Some(profile.company_name.clone()),
            currency_name: Some(profile.currency.clone()),
            currency_symbol: Some(currency_symbol.to_string()),
            active: Some(profile.is_active),
            description: Some(profile.description.clone()),
            homepage_url: Some(profile.website.clone()),
            weighted_shares_outstanding: Some(shares_outstanding),
            extra: std::collections::HashMap::new(),
        })
    }

    pub async fn get_exchange_rates(&self) -> Result<Vec<ExchangeRate>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://financialmodelingprep.com/api/v3/quotes/forex?apikey={}",
            self.api_key
        );

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        let rates: Vec<ExchangeRate> = response.json().await?;
        Ok(rates)
    }
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRate {
    pub name: String,
    pub price: f64,
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

impl PolygonClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn get_details(&self, ticker: &str, date: NaiveDate) -> Result<Details> {
        if ticker.is_empty() {
            anyhow::bail!("ticker empty");
        }

        let url = format!(
            "https://api.polygon.io/v3/reference/tickers/{}?date={}",
            ticker,
            date.format("%Y-%m-%d")
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to get response text")?;

        if !status.is_success() {
            anyhow::bail!("API error: {} - {}", status, text);
        }

        // Try to parse the response, if it fails, print the raw response for debugging
        match serde_json::from_str::<PolygonResponse>(&text) {
            Ok(polygon_response) => Ok(polygon_response.results),
            Err(e) => {
                eprintln!("Failed to parse response: {}", e);
                eprintln!("Raw response: {}", text);
                Err(e).context("Failed to parse response")
            }
        }
    }
}

pub async fn get_details_eu(ticker: &str) -> Result<Details> {
    let api_key = env::var("FIANANCIALMODELINGPREP_API_KEY").expect("FIANANCIALMODELINGPREP_API_KEY must be set");
    let client = FMPClient::new(api_key);
    client.get_details(ticker).await
}
