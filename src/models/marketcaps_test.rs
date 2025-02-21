use super::*;
use crate::db;
use std::collections::HashMap;
use serde_json::json;
use std::fs;
use csv::Reader;

#[tokio::test]
async fn test_market_caps_operations() -> Result<()> {
    let pool = db::create_test_pool().await?;

    // Test data with different market caps
    let test_data = vec![
        ("LVMH", 364214008782.0, "EUR"),
        ("NIKE", 150000000000.0, "USD"),
        ("ADDYY", 30000000000.0, "EUR"),
        ("PUMA", 5000000000.0, "EUR"),
        ("COLM", 4000000000.0, "USD"),
    ];

    let timestamp = chrono::Local::now().timestamp();
    let rate_map = HashMap::from([
        ("USD/EUR".to_string(), 0.85),
        ("EUR/USD".to_string(), 1.18),
    ]);

    // Store multiple test records
    for (ticker, market_cap, currency) in test_data {
        let details = Details {
            ticker: ticker.to_string(),
            market_cap: Some(market_cap),
            name: Some(format!("Test Company {}", ticker)),
            currency_name: Some(currency.to_string()),
            currency_symbol: Some(if currency == "USD" { "$" } else { "€" }.to_string()),
            active: Some(true),
            description: Some(format!("Test Description for {}", ticker)),
            homepage_url: Some(format!("https://{}.com", ticker.to_lowercase())),
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

        store_market_cap(&pool, &details, &rate_map, timestamp).await?;
    }

    // Test getting market caps
    let market_caps = get_market_caps(&pool).await?;
    assert_eq!(market_caps.len(), 5, "Should have 5 market cap records");
    
    // Verify market caps are ordered correctly
    let (highest_mc, highest_details) = &market_caps[0];
    assert!(highest_mc > &0.0);
    assert_eq!(highest_details[0], "LVMH");

    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output")?;

    // Test exporting market caps
    export_market_caps(&pool).await?;
    export_top_100_active(&pool).await?;

    // Get the latest output file
    let output_dir = std::path::Path::new("output");
    let latest_file = fs::read_dir(output_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().to_string_lossy().contains("top_100_active_")
                && entry.path().extension().map_or(false, |ext| ext == "csv")
        })
        .max_by_key(|entry| entry.metadata().unwrap().modified().unwrap())
        .expect("No output file found");

    // Read and validate the CSV file
    let mut rdr = Reader::from_path(latest_file.path())?;
    let headers = rdr.headers()?.clone();
    
    // Check required headers
    let required_headers = vec![
        "Symbol", "Ticker", "Name", "Market Cap (Original)", "Original Currency",
        "Market Cap (EUR)", "Market Cap (USD)", "Exchange", "Active", "Description",
        "Homepage URL", "Employees", "Timestamp"
    ];
    
    for header in required_headers {
        assert!(headers.iter().any(|h| h == header), "Missing header: {}", header);
    }

    // Count the number of records
    let record_count = rdr.records().count();
    assert_eq!(record_count, 5, "CSV should contain 5 records");

    // Validate data types and values in each row
    for result in rdr.into_records() {
        let record = result?;
        
        // Check Symbol and Ticker match
        assert_eq!(record.get(0).unwrap(), record.get(1).unwrap(), "Symbol should match Ticker");
        
        // Check Market Cap values are numeric and positive
        let original_mc: f64 = record.get(3).unwrap().parse()?;
        let eur_mc: f64 = record.get(5).unwrap().parse()?;
        let usd_mc: f64 = record.get(6).unwrap().parse()?;
        assert!(original_mc > 0.0, "Market Cap (Original) should be positive");
        assert!(eur_mc > 0.0, "Market Cap (EUR) should be positive");
        assert!(usd_mc > 0.0, "Market Cap (USD) should be positive");
        
        // Check Active is boolean
        assert!(record.get(8).unwrap() == "true" || record.get(8).unwrap() == "false", 
            "Active should be boolean");
        
        // Check Homepage URL format
        let url = record.get(10).unwrap();
        assert!(url.starts_with("http://") || url.starts_with("https://"), 
            "Homepage URL should start with http:// or https://");
    }

    Ok(())
}
