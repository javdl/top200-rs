use super::*;
use crate::db;
use std::collections::HashMap;
use serde_json::json;

#[tokio::test]
async fn test_market_caps_operations() -> Result<()> {
    let pool = db::create_test_pool().await?;

    // Test storing market cap data
    let details = Details {
        ticker: "TEST".to_string(),
        market_cap: Some(1000000.0),
        name: Some("Test Company".to_string()),
        currency_name: Some("USD".to_string()),
        currency_symbol: Some("$".to_string()),
        active: Some(true),
        description: Some("Test Description".to_string()),
        homepage_url: Some("https://test.com".to_string()),
        weighted_shares_outstanding: Some(1000.0),
        employees: Some("100".to_string()),
        revenue: Some(500000.0),
        revenue_usd: Some(500000.0),
        timestamp: Some("2025-02-21".to_string()),
        working_capital_ratio: Some(1.5),
        quick_ratio: Some(1.2),
        eps: Some(2.5),
        pe_ratio: Some(20.0),
        debt_equity_ratio: Some(0.5),
        roe: Some(0.15),
        extra: HashMap::from([
            ("roa".to_string(), json!(0.1)),
            ("price_to_book_ratio".to_string(), json!(2.0)),
            ("price_to_sales_ratio".to_string(), json!(3.0)),
            ("enterprise_value".to_string(), json!(1500000.0)),
        ]),
    };

    let rate_map = HashMap::from([
        ("USD/EUR".to_string(), 0.85),
        ("EUR/USD".to_string(), 1.18),
    ]);

    let timestamp = chrono::Local::now().timestamp();
    store_market_cap(&pool, &details, &rate_map, timestamp).await?;

    // Test getting market caps
    let market_caps = get_market_caps(&pool).await?;
    assert!(!market_caps.is_empty());
    let (market_cap, details) = &market_caps[0];
    assert!(market_cap > &0.0);
    assert!(!details.is_empty());

    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Test exporting market caps
    export_market_caps(&pool).await?;
    export_top_100_active(&pool).await?;

    Ok(())
}
