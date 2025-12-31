use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub server: ServerSettings,
    pub logging: LoggingSettings,
    pub database: DatabaseSettings,
    pub middleware: MiddlewareSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    #[serde(skip_deserializing)]
    pub environment: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    // MySQL 连接信息
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub charset: String,
    pub collation: String,
    pub prefix: String,
    
    // 连接池配置
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    
    // 日志配置
    pub enable_logging: bool,
    pub slow_query_log: bool,
    pub slow_query_threshold: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareSettings {
    pub trace: bool,
    pub cors: bool,
    pub compression: bool,
    pub catch_panic: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppSettings {
                name: "axum-learn".to_string(),
                environment: "development".to_string(),
                debug: true,
            },
            server: ServerSettings {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            logging: LoggingSettings {
                level: "debug".to_string(),
                format: "pretty".to_string(),
            },
            database: DatabaseSettings {
                driver: "mysql".to_string(),
                host: "localhost".to_string(),
                port: 3306,
                database: "axum_learn".to_string(),
                username: "root".to_string(),
                password: "".to_string(),
                charset: "utf8".to_string(),
                collation: "utf8_unicode_ci".to_string(),
                prefix: "".to_string(),
                max_connections: 25,
                min_connections: 5,
                connect_timeout: 30,
                acquire_timeout: 30,
                idle_timeout: 600,
                max_lifetime: 1800,
                enable_logging: true,
                slow_query_log: false,
                slow_query_threshold: 1000,
            },
            middleware: MiddlewareSettings {
                trace: true,
                cors: true,
                compression: true,
                catch_panic: true,
            },
        }
    }
}

impl AppConfig {
    pub fn load_with_env(env_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::builder()
            // 环境特定配置文件（基础配置）
            .add_source(File::with_name(&format!("config/{}", env_name)).required(false))
            // 环境变量覆盖（优先级最高）
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .prefix_separator("_")
                    .try_parsing(true)
            )
            .build()?;

        let mut settings: AppConfig = config.try_deserialize()?;
        settings.app.environment = env_name.to_string();
        Ok(settings)
    }

    pub fn get_log_filter(&self) -> String {
        // 基于配置的日志级别生成过滤字符串
        let base_level = if self.app.debug {
            "debug"
        } else {
            &self.logging.level
        };

        format!("axum_learn={},tower_http={},config={}", 
                base_level, 
                self.logging.level,
                base_level)
    }

    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

impl DatabaseSettings {
    pub fn build_connect_options(&self) -> ConnectOptions {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        
        // URL 编码密码中的特殊字符
        let encoded_password = utf8_percent_encode(&self.password, NON_ALPHANUMERIC).to_string();
        
        let url = format!(
            "mysql://{}:{}@{}:{}/{}?charset={}",
            self.username, encoded_password, self.host, 
            self.port, self.database, self.charset
        );
        
        let mut opt = ConnectOptions::new(url);
        opt.max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect_timeout(Duration::from_secs(self.connect_timeout))
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .max_lifetime(Duration::from_secs(self.max_lifetime))
            .acquire_timeout(Duration::from_secs(self.acquire_timeout))
            .sqlx_logging(self.enable_logging);
            
        opt
    }
    
    pub async fn connect(&self) -> Result<DatabaseConnection, sea_orm::DbErr> {
        let options = self.build_connect_options();
        sea_orm::Database::connect(options).await
    }
    
    pub fn get_database_url(&self) -> String {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        
        // URL 编码密码中的特殊字符
        let encoded_password = utf8_percent_encode(&self.password, NON_ALPHANUMERIC).to_string();
        
        format!(
            "mysql://{}:{}@{}:{}/{}?charset={}",
            self.username, encoded_password, self.host, 
            self.port, self.database, self.charset
        )
    }
}