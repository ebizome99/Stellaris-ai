//! 日志初始化模块

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// 初始化日志系统
pub fn init_logging() {
    let log_dir = dirs::home_dir()
        .map(|h| h.join(".stellaris-ai").join("logs"))
        .unwrap_or_else(|| std::path::PathBuf::from("./logs"));
    
    let _ = std::fs::create_dir_all(&log_dir);
    
    let file_appender = tracing_appender::rolling::daily(&log_dir, "stellaris-ai.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "stellaris_ai=info,debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();
    
    tracing::info!("日志系统初始化完成, 日志目录: {:?}", log_dir);
}
