use anyhow::Result;
use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, program_pack::Pack, pubkey::Pubkey};
use spl_token::state::Mint;
use std::{env, fmt, path::Path, str::FromStr};
use time::{macros::format_description, UtcOffset};
use tracing::{debug, info, instrument, warn};
use tracing_subscriber::{
    fmt::{format::json, time::OffsetTime},
    EnvFilter,
};

mod error;

pub use error::*;

/// 定义 ToPubkey trait，用于将不同类型转换为 Solana 的公钥（Pubkey）
pub trait ToPubkey {
    fn to_pubkey(&self) -> Result<Pubkey>;
}

/// 为 str 类型实现 ToPubkey trait
impl ToPubkey for str {
    fn to_pubkey(&self) -> Result<Pubkey> {
        Pubkey::from_str(self).map_err(|e| e.into())
    }
}

/// 为 String 类型实现 ToPubkey trait
impl ToPubkey for String {
    fn to_pubkey(&self) -> Result<Pubkey> {
        self.as_str().to_pubkey()
    }
}

/// 为 Pubkey 类型实现 ToPubkey trait（直接返回自身）
impl ToPubkey for Pubkey {
    fn to_pubkey(&self) -> Result<Pubkey> {
        Ok(*self)
    }
}

/// 为实现了 ToPubkey trait 的类型的引用实现 ToPubkey trait
impl<T: ToPubkey + ?Sized> ToPubkey for &T {
    fn to_pubkey(&self) -> Result<Pubkey> {
        (*self).to_pubkey()
    }
}

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
/// 返回 `Result<RpcClient, ClientError>`，其中包含初始化的 RPC 客户端，或在出错时返回错误。
///
/// # 错误
///
/// 如果无法从环境变量获取 RPC URL 或创建客户端失败，将返回错误。
pub fn init_rpc_client(
    commitment_config: CommitmentConfig,
) -> Result<RpcClient, solana_client::client_error::ClientError> {
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

/// 格式化 Metadata 为易读的 JSON 字符串
pub fn format_metadata(metadata: &Metadata) -> String {
    let metadata_json = serde_json::json!({
        "key": format!("{:?}", metadata.key),
        "update_authority": metadata.update_authority.to_string(),
        "mint": metadata.mint.to_string(),
        "name": metadata.name.trim_matches(char::from(0)),
        "symbol": metadata.symbol.trim_matches(char::from(0)),
        "uri": metadata.uri.trim_matches(char::from(0)),
        "seller_fee_basis_points": metadata.seller_fee_basis_points,
        "primary_sale_happened": metadata.primary_sale_happened,
        "is_mutable": metadata.is_mutable,
        "edition_nonce": metadata.edition_nonce,
        "token_standard": format!("{:?}", metadata.token_standard),
    });

    serde_json::to_string_pretty(&metadata_json).unwrap_or_else(|_| "格式化失败".to_string())
}

/// 获取代币信息
///
/// 该函数通过给定的代币账户地址获取代币的元数据和小数位数。
///
/// # 参数
/// * `token_account` - 代币账户或 mint 地址
/// * `is_mint` - 是否直接是 mint 地址
///
/// # 返回值
/// * `Result<(Metadata, u8)>` - 代币元数据和小数位数
#[instrument(skip(rpc_client, token_account))]
pub fn fetch_token_info<T>(rpc_client: &RpcClient, token_account: T) -> Result<(Metadata, Mint)>
where
    T: ToPubkey + fmt::Debug,
{
    let token_pubkey = token_account.to_pubkey()?;
    info!("正在获取代币信息，账户: {}", token_pubkey);

    let (mint_pda, _) = Metadata::find_pda(&token_pubkey);
    debug!("正在获取 PDA 的元数据: {}", mint_pda);

    let m_data = rpc_client.get_account_data(&mint_pda)?;
    let metadata = Metadata::safe_deserialize(&m_data)?;

    debug!("正在获取代币小数位数");
    let data = rpc_client.get_account_data(&token_pubkey)?;
    let mint = Mint::unpack(&data)?;

    Ok((metadata, mint))
}

pub fn extract_token_info(info: &serde_json::Value) -> Option<(String, u64, String)> {
    let mint = info.get("mint")?.as_str()?.to_string();
    let amount = info
        .get("tokenAmount")?
        .get("amount")?
        .as_str()?
        .parse::<u64>()
        .ok()?;
    let symbol = info.get("symbol")?.as_str()?.to_string();

    Some((mint, amount, symbol))
}
