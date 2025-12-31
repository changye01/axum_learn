use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QuerySelect, QueryOrder, PaginatorTrait};
use crate::entities::account;
use crate::models::account::{AccountResponse, AccountsListResponse, AccountSummaryResponse};

// Service 改为无状态（空结构体）
pub struct AccountService;

impl AccountService {
    // 获取所有账号（带分页）
    pub async fn get_all_accounts(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> Result<AccountsListResponse, sea_orm::DbErr> {
        let offset = (page - 1) * page_size;

        let total = account::Entity::find()
            .count(db)
            .await?;

        let accounts = account::Entity::find()
            .order_by_asc(account::Column::Id)
            .offset(offset)
            .limit(page_size)
            .all(db)
            .await?;

        let total_pages = (total + page_size - 1) / page_size;

        let response_data: Vec<AccountResponse> = accounts
            .into_iter()
            .map(AccountResponse::from)
            .collect();

        Ok(AccountsListResponse {
            total,
            data: response_data,
            page,
            page_size,
            total_pages,
        })
    }

    // 获取启用的账号
    pub async fn get_enabled_accounts(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> Result<AccountsListResponse, sea_orm::DbErr> {
        let offset = (page - 1) * page_size;

        let total = account::Entity::find()
            .filter(account::Column::IsEnable.eq(1))
            .count(db)
            .await?;

        let accounts = account::Entity::find()
            .filter(account::Column::IsEnable.eq(1))
            .order_by_asc(account::Column::Id)
            .offset(offset)
            .limit(page_size)
            .all(db)
            .await?;

        let total_pages = (total + page_size - 1) / page_size;

        let response_data: Vec<AccountResponse> = accounts
            .into_iter()
            .map(AccountResponse::from)
            .collect();

        Ok(AccountsListResponse {
            total,
            data: response_data,
            page,
            page_size,
            total_pages,
        })
    }

    // 获取禁用的账号
    pub async fn get_disabled_accounts(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> Result<AccountsListResponse, sea_orm::DbErr> {
        let offset = (page - 1) * page_size;

        let total = account::Entity::find()
            .filter(account::Column::IsEnable.eq(0))
            .count(db)
            .await?;

        let accounts = account::Entity::find()
            .filter(account::Column::IsEnable.eq(0))
            .order_by_asc(account::Column::Id)
            .offset(offset)
            .limit(page_size)
            .all(db)
            .await?;

        let total_pages = (total + page_size - 1) / page_size;

        let response_data: Vec<AccountResponse> = accounts
            .into_iter()
            .map(AccountResponse::from)
            .collect();

        Ok(AccountsListResponse {
            total,
            data: response_data,
            page,
            page_size,
            total_pages,
        })
    }

    // 获取账号统计
    pub async fn get_accounts_summary(
        db: &DatabaseConnection,
    ) -> Result<AccountSummaryResponse, sea_orm::DbErr> {
        let all_accounts = account::Entity::find()
            .all(db)
            .await?;

        let total_accounts = all_accounts.len() as u64;

        let enabled_accounts = all_accounts
            .iter()
            .filter(|account| account.is_enable == 1)
            .count() as u64;

        let disabled_accounts = all_accounts
            .iter()
            .filter(|account| account.is_enable == 0)
            .count() as u64;

        let companies: Vec<String> = all_accounts
            .into_iter()
            .map(|account| account.company_name)
            .filter(|company| !company.is_empty())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        Ok(AccountSummaryResponse {
            total_accounts,
            enabled_accounts,
            disabled_accounts,
            companies,
        })
    }
}