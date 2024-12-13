use std::str::FromStr;

use anyhow::Result;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::account::ReadableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Account;

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
