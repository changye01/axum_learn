use axum::{extract::{State, Query}, http::StatusCode, Json};
use sea_orm::DatabaseConnection;
use crate::models::account::{AccountsListResponse, PaginationParams, AccountSummaryResponse};
use crate::service::account_service::AccountService;

// 获取所有账号数据（带分页）
pub async fn list_all_accounts(
    State(db): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<AccountsListResponse>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20).min(100).max(1);

    AccountService::get_all_accounts(&db, page, page_size)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// 获取启用的账号（带分页）
pub async fn list_enabled_accounts(
    State(db): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<AccountsListResponse>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20).min(100).max(1);

    AccountService::get_enabled_accounts(&db, page, page_size)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// 获取未启用的账号（带分页）
pub async fn list_disabled_accounts(
    State(db): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<AccountsListResponse>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20).min(100).max(1);

    AccountService::get_disabled_accounts(&db, page, page_size)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// 获取账号统计信息
pub async fn get_accounts_summary(
    State(db): State<DatabaseConnection>,
) -> Result<Json<AccountSummaryResponse>, StatusCode> {
    AccountService::get_accounts_summary(&db)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}