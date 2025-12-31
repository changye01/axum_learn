use std::sync::Arc;
use std::time::Duration;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<AppConfig>,
}

// 实现 FromRef，让 Handler 可以自动提取 DatabaseConnection
impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

// 如果需要，也可以提取 AppConfig
impl FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, sea_orm::DbErr> {
        // 构建连接池配置
        let mut opt = sea_orm::ConnectOptions::new(config.database.get_database_url());
        opt.max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .connect_timeout(Duration::from_secs(config.database.connect_timeout))
            .acquire_timeout(Duration::from_secs(config.database.acquire_timeout))
            .idle_timeout(Duration::from_secs(config.database.idle_timeout))
            .max_lifetime(Duration::from_secs(config.database.max_lifetime))
            .sqlx_logging(config.database.enable_logging);
        
        // 创建数据库连接池
        let db = sea_orm::Database::connect(opt).await?;
        
        Ok(Self {
            db,
            config: Arc::new(config),
        })
    }
}