// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use tokio_postgres::Client;

#[derive(Debug)]
pub struct TickerDetails {
    pub ticker: String,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
    pub employees: Option<String>,
}

/// Update specific ticker details in the company_details table
pub async fn update_ticker_details(client: &mut Client, details: &TickerDetails) -> Result<()> {
    client
        .execute(
            r#"
            INSERT INTO company_details (ticker, description, homepage_url, employees, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            ON CONFLICT(ticker) DO UPDATE SET
                description = EXCLUDED.description,
                homepage_url = EXCLUDED.homepage_url,
                employees = EXCLUDED.employees,
                updated_at = NOW()
            "#,
            &[
                &details.ticker,
                &details.description,
                &details.homepage_url,
                &details.employees,
            ],
        )
        .await?;
    Ok(())
}