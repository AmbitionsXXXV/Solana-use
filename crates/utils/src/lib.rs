use anyhow::Result;
use std::{env, path::Path};
use time::{macros::format_description, UtcOffset};
use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};

/// 初始化 tracing 日志系统
///
/// 该函数设置和初始化 tracing 日志系统，使用环境变量配置的过滤器。
pub fn init_tracing() {
    // 从环境变量获取时区配置，默认东八区 (+8)
    let offset = env::var("TZ_OFFSET")
        .ok()
        .and_then(|x| x.parse::<i8>().ok())
        .unwrap_or(8);

    let timer = OffsetTime::new(
        UtcOffset::from_hms(offset, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );

    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

/// 加载环境变量
///
/// 从工作空间根目录的 .env 文件中加载环境变量
pub fn load_env() -> Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let workspace_root = Path::new(&manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("Failed to find workspace root");
    let dot_env_path = workspace_root.join(".env");

    dotenv::from_path(&dot_env_path)?;

    Ok(())
}
