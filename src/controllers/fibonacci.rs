use crate::{
    models::fibonacci::{FibonacciQuery, FibonacciResponse},
    service::fibonacci_service::FibonacciService,
    config::AppConfig,
    error::AppError,
};
use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::json;
use chrono::Utc;
use std::sync::Arc;

pub async fn fibonacci_controller(
    Query(query): Query<FibonacciQuery>,
) -> Result<Json<FibonacciResponse>, AppError> {
    let n = query.n.unwrap_or(10);

    // u64 最大支持的斐波那契数列索引
    if n > 93 {
        return Err(AppError::ValidationError("n must be <= 93 (u64 overflow protection)".to_string()));
    }

    let result = FibonacciService::get_fibonacci(n);
    Ok(Json(FibonacciResponse { n, result }))
}

pub async fn health_check(
    State(config): State<Arc<AppConfig>>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
        "app": {
            "name": config.app.name,
            "environment": config.app.environment,
            "debug": config.app.debug
        },
        "version": "0.1.0"
    })))
}
