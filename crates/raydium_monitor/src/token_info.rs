use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use mpl_token_metadata::accounts::Metadata;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::account::ReadableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Account;
use spl_token::state::Mint;
use tracing::{debug, info, instrument};

use crate::client::init_rpc_client;
use crate::model::ToPubkey;

/// 获取代币信息
///
/// 该函数通过给定的代币账户地址获取代币的元数据和小数位数。
///
/// # 参数
///
/// * `token_account` - 代币账户地址字符串
///
/// # 返回值
///
/// 返回 `Result<(Metadata, u8)>`，包含代币元数据和小数位数
#[instrument(skip(token_account))]
pub fn fetch_token_info<T>(token_account: T) -> Result<(Metadata, u8)>
where
    T: ToPubkey + fmt::Debug,
{
    let token_pubkey = token_account.to_pubkey()?;
    info!("正在获取代币信息，账户: {}", token_pubkey);
    // 步骤 1：创建 RPC 客户端
    let rpc_client = init_rpc_client(CommitmentConfig::confirmed())?;

    // 步骤 2：查找元数据 PDA
    let (mint_pda, _) = Metadata::find_pda(&token_pubkey);
    debug!("正在获取 PDA 的元数据: {}", mint_pda);
    // 步骤 3：获取元数据账户数据
    let m_data = rpc_client.get_account_data(&mint_pda)?;
    let metadata = Metadata::safe_deserialize(&m_data)?;

    debug!("正在获取代币小数位数");
    // 步骤 4：获取代币账户数据
    let data = rpc_client.get_account_data(&token_pubkey)?;
    let mint = Mint::unpack(&data)?;

    info!("成功获取代币信息");
    Ok((metadata, mint.decimals))
}

pub async fn get_token_addresses(
    rpc_client: &RpcClient,
    accounts: &[String],
) -> Result<(Option<Pubkey>, Option<Pubkey>)> {
    let config = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        commitment: Some(CommitmentConfig::confirmed()),
        ..RpcAccountInfoConfig::default()
    };

    let source_token_account_info = rpc_client
        .get_account_with_config(&Pubkey::from_str(&accounts[15])?, config.clone())?
        .value;
    let destination_token_account_info = rpc_client
        .get_account_with_config(&Pubkey::from_str(&accounts[16])?, config.clone())?
        .value;

    let source_token_address = source_token_account_info
        .and_then(|info| Account::unpack_unchecked(info.data()).ok())
        .map(|account| account.mint);

    let destination_token_address = destination_token_account_info
        .and_then(|info| Account::unpack_unchecked(info.data()).ok())
        .map(|account| account.mint);

    Ok((source_token_address, destination_token_address))
}
