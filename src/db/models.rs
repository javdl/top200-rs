#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub exchange_rate_usd: Option<f64>,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
} 