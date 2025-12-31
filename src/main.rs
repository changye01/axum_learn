mod config;
mod controllers;
mod entities;
mod error;
mod models;
mod routes;
mod service;
mod state;

use axum::{routing::get, Router};
use clap::Parser;
use config::AppConfig;
use routes::api::api_routes;
use state::AppState;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer,
};


#[derive(Parser, Debug)]
#[command(name = "axum-learn")]
#[command(about = "Axum 学习项目", long_about = None)]
struct Args {
    /// 运行环境 (development, staging, production)
    #[arg(short, long, default_value = "development")]
    env: String,

    /// 服务器监听地址 (覆盖配置文件)
    #[arg(short = 'H', long)]
    host: Option<String>,

    /// 服务器监听端口 (覆盖配置文件)
    #[arg(short = 'P', long)]
    port: Option<u16>,

    /// 配置文件路径 (可选)
    #[arg(short = 'c', long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let args = Args::parse();

    // 加载配置
    let mut config = AppConfig::load_with_env(&args.env)?;

    // 覆盖配置
    if let Some(host) = args.host {
        config.server.host = host;
    }
    if let Some(port) = args.port {
        config.server.port = port;
    }

    // 初始化结构化日志
    tracing_subscriber::fmt()
        .with_env_filter(config.get_log_filter())
        .init();

    // 打印配置信息（所有环境）
    // println!("🔧 Configuration loaded: {} env={}",
    //     serde_json::to_string_pretty(&config).unwrap_or_else(|_| "Failed to serialize".to_string()),
    //     config.app.environment
    // );

    let state = AppState::new(config.clone()).await?;
    


    // 记录启用的中间件
    let mut enabled_middleware = Vec::new();

    // 构建应用，根据配置动态添加中间件
    let app = {
        let mut router = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .merge(api_routes());

        // 根据配置添加中间件（按重要性顺序）
        if config.middleware.compression {
            router = router.layer(CompressionLayer::new());
            enabled_middleware.push("Compression");
        }
        
        if config.middleware.cors {
            router = router.layer(CorsLayer::permissive());
            enabled_middleware.push("CORS");
        }
        
        if config.middleware.trace {
            router = router.layer(TraceLayer::new_for_http());
            enabled_middleware.push("Trace");
        }
        
        if config.middleware.catch_panic {
            router = router.layer(CatchPanicLayer::new());
            enabled_middleware.push("CatchPanic");
        }

        router.with_state(state)
    };

    if !enabled_middleware.is_empty() {
        tracing::info!("Enabled middleware: {}", enabled_middleware.join(", "));
    } else {
        tracing::info!("No middleware enabled");
    }

    let addr = &config.get_server_address();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(
        "Server listening on {} (env: {})",
        addr,
        config.app.environment
    );
    tracing::info!(
        "App name: {} | Debug: {}",
        config.app.name,
        config.app.debug
    );

    axum::serve(listener, app).await?;
    Ok(())
}
