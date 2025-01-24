use super::*;
use crate::models::Details;
use std::collections::HashMap;
use chrono::Utc;
use mockall::automock;
use crate::api::{FMPClient, ExchangeRate};

#[automock]
trait FMPClientTrait {
    async fn get_exchange_rates(&self) -> Result<Vec<ExchangeRate>>;
}

#[tokio::test]
async fn test_market_cap_operations() -> Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Setup test data
    let mut rate_map = HashMap::new();
    rate_map.insert("USD/EUR".to_string(), 0.85);
    rate_map.insert("EUR/USD".to_string(), 1.18);
    
    let details = Details {
        ticker: "TEST".to_string(),
        name: Some("Test Company".to_string()),
        market_cap: Some(1000000.0),
        currency_symbol: Some("USD".to_string()),
        currency_name: Some("US Dollar".to_string()),
        description: Some("Test Description".to_string()),
        homepage_url: Some("https://test.com".to_string()),
        employees: Some("100".to_string()),
        active: Some(true),
        weighted_shares_outstanding: Some(1000000.0),
        revenue: Some(500000.0),
        revenue_usd: Some(500000.0),
        timestamp: Some("2025-01-24".to_string()),
        working_capital_ratio: Some(1.5),
        quick_ratio: Some(1.2),
        eps: Some(2.5),
        pe_ratio: Some(20.0),
        debt_equity_ratio: Some(0.5),
        roe: Some(15.0),
        extra: HashMap::new(),
    };

    let timestamp = Utc::now().timestamp();

    // Test store_market_cap
    store_market_cap(&pool, &details, &rate_map, timestamp).await?;

    // Test get_market_caps
    let market_caps = get_market_caps(&pool).await?;
    assert!(!market_caps.is_empty());
    let (market_cap_eur, data) = &market_caps[0];
    assert!(market_cap_eur > &0.0);
    assert_eq!(data[0], "TEST"); // ticker
    assert_eq!(data[2], "Test Company"); // name
    assert_eq!(data[4], "USD"); // original_currency

    // Test export functions (just verify they don't fail)
    export_market_caps(&pool).await?;
    export_top_100_active(&pool).await?;

    Ok(())
}

#[tokio::test]
async fn test_marketcaps_main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Create mock FMP client
    let mut mock_client = MockFMPClientTrait::new();
    mock_client
        .expect_get_exchange_rates()
        .returning(|| Ok(vec![
            ExchangeRate {
                name: Some("EUR/USD".to_string()),
                price: Some(1.18),
                changes_percentage: None,
                change: None,
                day_low: None,
                day_high: None,
                year_high: None,
                year_low: None,
                market_cap: None,
                price_avg_50: None,
                price_avg_200: None,
                volume: None,
                avg_volume: None,
                exchange: Some("FOREX".to_string()),
                open: None,
                previous_close: None,
                timestamp: 1701956301,
            },
            ExchangeRate {
                name: Some("USD/EUR".to_string()),
                price: Some(0.85),
                changes_percentage: None,
                change: None,
                day_low: None,
                day_high: None,
                year_high: None,
                year_low: None,
                market_cap: None,
                price_avg_50: None,
                price_avg_200: None,
                volume: None,
                avg_volume: None,
                exchange: Some("FOREX".to_string()),
                open: None,
                previous_close: None,
                timestamp: 1701956301,
            },
        ]));

    // TODO: Implement this test after adding dependency injection for FMP client
    // For now, we'll skip the test since we need to modify the main function
    // to accept an FMP client as a parameter

    Ok(())
}
