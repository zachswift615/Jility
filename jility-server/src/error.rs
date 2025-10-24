use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<jility_core::CoreError> for ApiError {
    fn from(err: jility_core::CoreError) -> Self {
        match err {
            jility_core::CoreError::NotFound(msg) => ApiError::NotFound(msg),
            jility_core::CoreError::InvalidInput(msg) => ApiError::InvalidInput(msg),
            jility_core::CoreError::Database(err) => ApiError::Database(err),
            jility_core::CoreError::Serialization(err) => {
                ApiError::Internal(format!("Serialization error: {}", err))
            }
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            ApiError::InvalidInput(_) | ApiError::Validation(_) => {
                (StatusCode::BAD_REQUEST, "invalid_input")
            }
            ApiError::Database(_) | ApiError::Internal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
            }
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            details: None,
        });

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
