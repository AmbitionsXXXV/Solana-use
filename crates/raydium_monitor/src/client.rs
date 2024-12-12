use std::env;
use std::str::FromStr;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};

use tracing::{debug, info, instrument, warn};

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

/// 异步获取交易详情
///
/// 该函数通过给定的交易签名从 Solana 网络获取交易详情。
///
/// # 参数
///
/// * `signature` - 交易的签名字符串
///
/// # 返回值
///
/// 返回 `Result<EncodedConfirmedTransactionWithStatusMeta>`，
/// 其中包含编码后的确认交易及其元数据，或者在出错时返回错误。
///
/// # 错误
///
/// 如果无法连接到 RPC 节点、解析签名或获取交易详情失败，将返回错误。
#[instrument(skip(signature), fields(signature = %signature))]
pub async fn get_transaction_details(
    signature: &str,
) -> Result<EncodedConfirmedTransactionWithStatusMeta> {
    // 步骤 1：设置 RPC 客户端
    // 使用 "confirmed" 提交配置初始化 RPC 客户端
    let client = init_rpc_client(CommitmentConfig::confirmed())?;

    // 步骤 2：解析交易签名
    // 将输入的字符串签名转换为 Solana 的 Signature 类型
    let sign = Signature::from_str(signature)?;
    info!("正在获取交易详情");

    // 步骤 3：配置交易查询参数
    // 创建 RpcTransactionConfig，指定查询参数
    let config = RpcTransactionConfig {
        commitment: Some(CommitmentConfig::confirmed()), // 使用 "confirmed" 提交级别
        max_supported_transaction_version: Some(0),      // 设置支持的最大交易版本
        encoding: Some(UiTransactionEncoding::JsonParsed), // 使用 JSON 解析编码
    };

    // 步骤 4：获取交易详情
    // 使用 RPC 客户端的 get_transaction_with_config 方法获取交易详情
    let tx = client.get_transaction_with_config(&sign, config)?;

    debug!("成功获取交易详情");
    Ok(tx)
}
