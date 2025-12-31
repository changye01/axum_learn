use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Service unavailable: {0}")]
    ServiceError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
            AppError::ServiceError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "service_error", msg)
            }
        };

        let body = Json(json!({
            "error": {
                "type": error_type,
                "message": message,
                "timestamp": Utc::now().to_rfc3339()
            },
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
