use crate::{TokenAccountError, TokenAccountResult};
use solana_sdk::{pubkey::Pubkey, signer::Signer, transaction::Transaction};
use spl_token::instruction::close_account;
use std::str::FromStr;

/// -- 执行账户关闭操作
///
/// 关闭指定的代币账户，回收租金。
///
/// # 参数
/// * `connection` - RPC 客户端连接
/// * `wallet` - 钱包密钥对
/// * `account_pubkey` - 要关闭的账户公钥
/// * `rent_lamports` - 账户当前的租金金额
///
/// # 返回
/// * `TokenAccountResult<(String, u64)>` - 成功返回 (交易签名, 租金金额)，失败返回错误
pub async fn execute_close_account(
    connection: &solana_client::rpc_client::RpcClient,
    wallet: &solana_sdk::signature::Keypair,
    account_pubkey: &Pubkey,
    rent_lamports: u64,
) -> TokenAccountResult<(String, u64)> {
    let instruction = close_account(
        &spl_token::id(),
        account_pubkey,
        &wallet.pubkey(),
        &wallet.pubkey(),
        &[&wallet.pubkey()],
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&wallet.pubkey()),
        &[wallet],
        connection
            .get_latest_blockhash()
            .map_err(TokenAccountError::from)?,
    );

    let signature = connection
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| TokenAccountError::TransactionError(e.to_string()))?;

    Ok((signature.to_string(), rent_lamports))
}

/// -- 创建批量关闭交易
///
/// 为多个账户创建一个批量关闭交易。
///
/// # 参数
/// * `connection` - RPC 客户端连接
/// * `wallet` - 钱包密钥对
/// * `accounts` - 要关闭的账户列表
///
/// # 返回
/// * `TokenAccountResult<(Transaction, f64)>` - 成功返回 (交易对象, 预计回收租金)
pub async fn create_batch_close_transaction(
    connection: &solana_client::rpc_client::RpcClient,
    wallet: &solana_sdk::signature::Keypair,
    accounts: &[crate::account_info::TokenAccountInfo],
) -> TokenAccountResult<(Transaction, f64)> {
    let mut instructions = Vec::new();
    let mut total_rent_recovered = 0.0;

    for account in accounts {
        let pubkey = Pubkey::from_str(&account.address)
            .map_err(|e| TokenAccountError::AccountParseError(e.to_string()))?;
        let instruction = close_account(
            &spl_token::id(),
            &pubkey,
            &wallet.pubkey(),
            &wallet.pubkey(),
            &[&wallet.pubkey()],
        )?;
        instructions.push(instruction);
        total_rent_recovered += account.rent_sol;
    }

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&wallet.pubkey()),
        &[wallet],
        connection.get_latest_blockhash()?,
    );

    Ok((transaction, total_rent_recovered))
}

/// -- 销毁代币
///
/// 销毁指定账户中的代币。
///
/// # 参数
/// * `connection` - RPC 客户端连接
/// * `wallet` - 钱包密钥对
/// * `account_pubkey` - 要销毁代币的账户公钥
/// * `mint_pubkey` - 代币的 Mint 地址
/// * `amount` - 要销毁的代币数量
///
/// # 返回
/// * `TokenAccountResult<String>` - 成功返回交易签名，失败返回错误
pub async fn burn_tokens(
    connection: &solana_client::rpc_client::RpcClient,
    wallet: &solana_sdk::signature::Keypair,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    amount: u64,
) -> TokenAccountResult<String> {
    let burn_instruction = spl_token::instruction::burn(
        &spl_token::id(),
        account_pubkey,
        mint_pubkey,
        &wallet.pubkey(),
        &[&wallet.pubkey()],
        amount,
    )?;

    let recent_blockhash = connection.get_latest_blockhash()?;
    let burn_tx = Transaction::new_signed_with_payer(
        &[burn_instruction],
        Some(&wallet.pubkey()),
        &[wallet],
        recent_blockhash,
    );

    let signature = connection
        .send_and_confirm_transaction(&burn_tx)
        .map_err(|e| TokenAccountError::TransactionError(e.to_string()))?;

    Ok(signature.to_string())
}
