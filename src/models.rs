use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub ticker: String,
    #[serde(rename = "market_cap")]
    pub market_cap: Option<f64>,
    pub name: Option<String>,
    #[serde(rename = "currency_name")]
    pub currency_name: Option<String>,
    #[serde(rename = "currency_symbol")]
    pub currency_symbol: Option<String>,
    pub active: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "homepage_url")]
    pub homepage_url: Option<String>,
    #[serde(rename = "weighted_shares_outstanding")]
    pub weighted_shares_outstanding: Option<f64>,
    // Add catch-all for other fields we don't care about
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonResponse {
    pub status: String,
    pub request_id: String,
    pub results: Details,
    // Add catch-all for other fields we don't care about
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FMPCompanyProfile {
    pub symbol: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "mktCap", default)]
    pub market_cap: f64,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "website", default)]
    pub website: String,
    #[serde(rename = "fullTimeEmployees")]
    pub shares_outstanding: Option<String>,
    #[serde(rename = "price", default)]
    pub price: f64,
    pub currency: String,
    #[serde(rename = "exchangeShortName")]
    pub exchange: String,
    #[serde(rename = "isActivelyTrading", default)]
    pub is_active: bool,
    // Add any other fields you need from the FMP API
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

fn default_string() -> String {
    "0".to_string()
}
