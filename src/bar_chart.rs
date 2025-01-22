use crate::api;
use crate::config;
use crate::models::Details;
use anyhow::Result;
use chrono::Local;
use plotters::prelude::*;
use std::sync::Arc;

pub async fn generate_bar_chart() -> Result<()> {
    // First fetch exchange rates and setup client
    let api_key = std::env::var("FINANCIALMODELINGPREP_API_KEY")
        .expect("FINANCIALMODELINGPREP_API_KEY must be set");
    let fmp_client = Arc::new(api::FMPClient::new(api_key));

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

    // Create a map of currency pairs to rates
    let mut rate_map = std::collections::HashMap::new();
    for rate in exchange_rates {
        if let (Some(name), Some(price)) = (rate.name, rate.price) {
            rate_map.insert(name, price);
        }
    }

    // Get config and tickers
    let config = config::load_config()?;
    let tickers = [config.non_us_tickers, config.us_tickers].concat();

    // Collect all stock details
    let mut stocks = Vec::new();
    for ticker in tickers {
        if let Ok(details) = fmp_client.get_details(&ticker, &rate_map).await {
            stocks.push(details);
        }
    }

    // Generate timestamp and output path
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let output_path = format!("output/market_bar_chart_{}.png", timestamp);

    // Create the visualization
    create_market_bar_chart(stocks, &output_path)?;
    println!("✅ Bar chart generated: {}", output_path);

    Ok(())
}

fn create_market_bar_chart(stocks: Vec<Details>, output_path: &str) -> Result<()> {
    // Sort stocks by market cap (descending) and take top 100
    let mut sorted_stocks: Vec<_> = stocks
        .into_iter()
        .filter(|s| s.market_cap.is_some() && s.name.is_some())
        .collect();
    sorted_stocks.sort_by(|a, b| b.market_cap.partial_cmp(&a.market_cap).unwrap());
    let top_stocks = sorted_stocks.into_iter().take(100).collect::<Vec<_>>();

    // Setup the drawing area
    let root_area = BitMapBackend::new(output_path, (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE)?;

    // Calculate the y-axis range (market caps in billions)
    let max_cap = top_stocks.first().and_then(|s| s.market_cap).unwrap_or(0.0) / 1_000_000_000.0;
    let y_range = 0.0..(max_cap * 1.1); // Add 10% padding

    let mut chart = ChartBuilder::on(&root_area)
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            "Top 100 Fashion & Luxury Companies by Market Cap",
            ("sans-serif", 30),
        )
        .build_cartesian_2d(
            0i32..100i32, // Explicitly using i32
            y_range,
        )?;

    // Configure the mesh
    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_desc("Market Cap (Billions EUR)")
        .draw()?;

    // Draw the bars
    for (i, stock) in top_stocks.iter().enumerate() {
        let cap_billions = stock.market_cap.unwrap_or(0.0) / 1_000_000_000.0;
        let i = i as i32; // Convert index to i32

        // Draw the bar
        chart.draw_series(std::iter::once(Rectangle::new(
            [(i, 0.0), (i + 1, cap_billions)],
            BLUE.mix(0.4).filled(),
        )))?;

        // Draw the label for top 10
        if i < 10 {
            if let Some(name) = &stock.name {
                chart.draw_series(std::iter::once(Text::new(
                    name.clone(),
                    (i, cap_billions + (max_cap * 0.02)),
                    ("sans-serif", 15).into_font(),
                )))?;
            }
        }
    }

    root_area.present()?;
    Ok(())
}
