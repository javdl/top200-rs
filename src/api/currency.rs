use axum::{
    extract::State,
    http::StatusCode,
    Json,
    Router,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::db::models::Currency;

#[derive(Serialize)]
pub struct CurrencyResponse {
    pub currencies: Vec<Currency>,
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(get_currencies))
        .route("/:code", get(get_currency_by_code))
}

async fn get_currencies(
    State(pool): State<SqlitePool>,
) -> Result<Json<CurrencyResponse>, AppError> {
    let currencies = sqlx::query_as!(
        Currency,
        r#"
        SELECT * FROM currencies
        ORDER BY code
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(CurrencyResponse { currencies }))
}

async fn get_currency_by_code(
    State(pool): State<SqlitePool>,
    axum::extract::Path(code): axum::extract::Path<String>,
) -> Result<Json<Currency>, AppError> {
    let currency = sqlx::query_as!(
        Currency,
        r#"
        SELECT * FROM currencies
        WHERE code = ?
        "#,
        code
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound("Currency not found".into()))?;

    Ok(Json(currency))
} 