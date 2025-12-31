use axum::{
    routing::{get, post},
    Router,
};
use crate::{
    controllers::{
        fibonacci::{fibonacci_controller, health_check},
        account_controller::{
            list_all_accounts, list_enabled_accounts, list_disabled_accounts, get_accounts_summary,
        },
    },
    state::AppState,
};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/api", api_internal_routes())
        .route("/health", get(health_check))
        // 新增账号管理端点
        .nest("/accounts", account_routes())
}

fn api_internal_routes() -> Router<AppState> {
    Router::new()
        .nest("/math", math_routes())
}

fn math_routes() -> Router<AppState> {
    Router::new()
        .nest("/fibonacci", fibonacci_routes())
}

fn fibonacci_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fibonacci_controller))
        // 未来可以扩展 POST 方法
        .route("/calculate", post(fibonacci_controller))
}

fn account_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_all_accounts))                    // 获取所有账号
        .route("/enabled", get(list_enabled_accounts))        // 获取启用的账号
        .route("/disabled", get(list_disabled_accounts))       // 获取未启用的账号
        .route("/summary", get(get_accounts_summary))           // 获取统计信息
}