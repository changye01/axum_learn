use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sfc_ozon_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    
    #[sea_orm(column_name = "account")]
    pub account: String, // 账号名称
    
    #[sea_orm(column_name = "client_id")]
    pub client_id: String,
    
    #[sea_orm(column_name = "api_key")]
    pub api_key: String,
    
    #[sea_orm(column_name = "currency_code")]
    pub currency_code: String, // 货币
    
    #[sea_orm(column_name = "company_name")]
    pub company_name: String, // 注册公司
    
    #[sea_orm(column_type = "Text")]
    pub data: String, // 配置数据
    
    #[sea_orm(column_name = "is_enable")]
    pub is_enable: u8, // 是否启用:0-未启用,1-启用
    
    #[sea_orm(column_name = "user_name")]
    pub user_name: String, // 负责人
    
    #[sea_orm(column_name = "create_time")]
    pub create_time: u32,
    
    #[sea_orm(column_name = "update_time")]
    pub update_time: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}