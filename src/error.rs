use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::NotFound(ref message) => (StatusCode::NOT_FOUND, message.clone()),
            AppError::Validation(ref message) => (StatusCode::BAD_REQUEST, message.clone()),
            AppError::Internal(ref message) => (StatusCode::INTERNAL_SERVER_ERROR, message.clone()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
} 