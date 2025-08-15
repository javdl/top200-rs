pub mod currency;
pub mod fmp_client;
pub mod polygon_client;
pub mod details;

pub use fmp_client::{FMPClient, FMPClientTrait, ExchangeRate};
pub use polygon_client::PolygonClient;
pub use details::{get_details_eu, get_details_eu_full};

use axum::{Router, routing::get};
use sqlx::SqlitePool;

// Add the HistoricalMarketCap struct from src/api.rs
#[derive(Debug, serde::Deserialize)]
pub struct HistoricalMarketCap {
    #[allow(dead_code)]
    pub ticker: String,
    pub name: String,
    pub market_cap_original: f64,
    pub original_currency: String,
    pub exchange: String,
    pub price: f64,
}

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .nest("/api/currencies", currency::routes())
        .with_state(pool)
} 