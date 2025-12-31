use serde::{Serialize, Deserialize};
use crate::entities::account;

#[derive(Serialize)]
pub struct AccountResponse {
    pub id: u32,
    pub account: String,
    pub client_id: String,
    pub api_key: String,
    pub currency_code: String,
    pub company_name: String,
    pub data: String,
    pub is_enable: u8,
    pub user_name: String,
    pub create_time: u32,
    pub update_time: u32,
}

impl From<account::Model> for AccountResponse {
    fn from(model: account::Model) -> Self {
        Self {
            id: model.id,
            account: model.account,
            client_id: model.client_id,
            api_key: model.api_key,
            currency_code: model.currency_code,
            company_name: model.company_name,
            data: model.data,
            is_enable: model.is_enable,
            user_name: model.user_name,
            create_time: model.create_time,
            update_time: model.update_time,
        }
    }
}

#[derive(Serialize)]
pub struct AccountsListResponse {
    pub total: u64,
    pub data: Vec<AccountResponse>,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct AccountSummaryResponse {
    pub total_accounts: u64,
    pub enabled_accounts: u64,
    pub disabled_accounts: u64,
    pub companies: Vec<String>,
}