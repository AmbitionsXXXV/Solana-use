use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{env, path::Path};
use time::{macros::format_description, UtcOffset};
use tracing::warn;
use tracing_subscriber::{
    fmt::{format::json, time::OffsetTime},
    EnvFilter,
};

/// 初始化 RPC 客户端
///
/// 该函数创建并返回一个 Solana RPC 客户端实例。
///
/// # 参数
///
/// * `commitment_config` - Solana 网络的提交配置
///
/// # 返回值
///
/// 返回 `Result<RpcClient>`，其中包含初始化的 RPC 客户端，或在出错时返回错误。
///
/// # 错误
///
/// 如果无法从环境变量获取 RPC URL 或创建客户端失败，将返回错误。
pub fn init_rpc_client(commitment_config: CommitmentConfig) -> Result<RpcClient> {
    // 尝试从环境变量获取 RPC URL，如果未设置则使用默认 mainnet URL
    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| {
        warn!("未设置 RPC_URL，使用默认的 mainnet URL");
        String::from("https://api.mainnet-beta.solana.com")
    });

    // 使用指定的 URL 和提交配置创建 RPC 客户端
    let rpc_client = RpcClient::new_with_commitment(rpc_url, commitment_config);

    Ok(rpc_client)
}

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
        .event_format(json().flatten_event(true))
        .pretty()
        .with_timer(timer)
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .with_line_number(false)
        .with_file(false)
        .with_target(false)
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
