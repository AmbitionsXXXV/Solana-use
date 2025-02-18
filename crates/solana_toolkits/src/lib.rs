use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, program_pack::Pack,
    pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};
use spl_token::{instruction::close_account, state::Account};
use std::{error::Error, fs::read_to_string, str::FromStr, thread, time::Duration};
use tracing::{error, info, warn};
use utils::{fetch_token_info, format_metadata, init_rpc_client};

// TokenAccountManager struct
pub struct TokenAccountManager {
    pub connection: RpcClient,
    pub wallet: Keypair,
}

/// -- 代币账户信息结构体
/// 存储单个代币账户的基本信息，包括地址、Mint、租金等
#[derive(Debug)]
pub struct TokenAccountInfo {
    pub address: String,    // -- 账户地址
    pub mint: String,       // -- 代币的 Mint 地址
    pub rent_lamports: u64, // -- 租金（以 lamports 为单位）
    pub rent_sol: f64,      // -- 租金（以 SOL 为单位）
}

/// -- 代币账户查询结果结构体
/// 包含查询到的所有代币账户统计信息
#[derive(Debug)]
pub struct TokenAccountsResult {
    pub total_accounts: usize,           // -- 总账户数量
    pub closable_accounts: usize,        // -- 可关闭的账户数量
    pub accounts: Vec<TokenAccountInfo>, // -- 可关闭账户列表
    pub total_rent_lamports: u64,        // -- 总租金（以 lamports 为单位）
    pub total_rent_sol: f64,             // -- 总租金（以 SOL 为单位）
}

/// -- 账户关闭结果结构体
/// 记录单个账户关闭操作的结果
#[derive(Debug)]
pub struct ClosureResult {
    pub success: bool,             // -- 操作是否成功
    pub signature: Option<String>, // -- 成功时的交易签名
    pub error: Option<String>,     // -- 失败时的错误信息
    pub account_address: String,   // -- 被关闭的账户地址
    pub rent_recovered: f64,       // -- 回收的租金数量（以 SOL 为单位）
}

/// -- 代币账户详细信息结构体
/// 存储代币账户的完整信息
#[derive(Debug)]
pub struct TokenAccountDetails {
    pub pubkey: String,     // -- 账户公钥
    pub balance: u64,       // -- 账户余额
    pub rent_lamports: u64, // -- 租金（以 lamports 为单位）
    pub rent_sol: f64,      // -- 租金（以 SOL 为单位）
    pub mint: String,       // -- 代币的 Mint 地址
    pub owner: String,      // -- 账户所有者地址
}

/// -- 销毁代币并回收账户结果结构体
#[derive(Debug)]
pub struct BurnAndCloseResult {
    pub success: bool,                   // -- 操作是否成功
    pub burn_signature: Option<String>,  // -- 销毁代币的交易签名
    pub close_signature: Option<String>, // -- 关闭账户的交易签名
    pub error: Option<String>,           // -- 失败时的错误信息
    pub account_address: String,         // -- 被操作的账户地址
    pub burned_amount: u64,              // -- 销毁的代币数量
    pub rent_recovered: f64,             // -- 回收的租金数量（以 SOL 为单位）
}

impl TokenAccountManager {
    /// -- 创建新的代币账户管理器实例
    ///
    /// # 参数
    /// * `wallet_key_path` - 钱包密钥文件路径
    ///
    /// # 返回
    /// * `Result<Self, Box<dyn Error>>` - 成功返回管理器实例，失败返回错误
    pub fn new(wallet_key_path: &str) -> Result<Self, Box<dyn Error>> {
        let connection = init_rpc_client(CommitmentConfig::confirmed())?;

        // -- 读取 JSON 并提取私钥字符串
        let key_str = read_to_string(wallet_key_path)?;
        let key_value: serde_json::Value = serde_json::from_str(&key_str)?;
        let private_key = key_value.as_str().ok_or("Invalid key format")?;

        let wallet = Keypair::from_base58_string(private_key);

        Ok(Self { connection, wallet })
    }

    /// -- 获取指定账户的详细信息
    ///
    /// # 参数
    /// * `account_pubkey` - 账户的公钥
    ///
    /// # 返回
    /// * `Result<TokenAccountDetails, Box<dyn Error>>` - 成功返回账户详情，失败返回错误
    pub async fn get_account_details(
        &self,
        account_pubkey: &Pubkey,
    ) -> Result<TokenAccountDetails, Box<dyn Error>> {
        let account_info = self.connection.get_account(account_pubkey)?;
        let token_account = Account::unpack(&account_info.data)?;

        Ok(TokenAccountDetails {
            pubkey: account_pubkey.to_string(),
            balance: token_account.amount,
            rent_lamports: account_info.lamports,
            rent_sol: account_info.lamports as f64 / LAMPORTS_PER_SOL as f64,
            mint: token_account.mint.to_string(),
            owner: token_account.owner.to_string(),
        })
    }

    /// -- 执行账户关闭操作
    ///
    /// # 参数
    /// * `account_pubkey` - 要关闭的账户公钥
    /// * `rent_lamports` - 账户当前的租金金额
    ///
    /// # 返回
    /// * `Result<(String, u64), Box<dyn Error>>` - 成功返回 (交易签名, 租金金额)，失败返回错误
    pub async fn execute_close_account(
        &self,
        account_pubkey: &Pubkey,
        rent_lamports: u64,
    ) -> Result<(String, u64), Box<dyn Error>> {
        let instruction = close_account(
            &spl_token::id(),
            account_pubkey,
            &self.wallet.pubkey(),
            &self.wallet.pubkey(),
            &[&self.wallet.pubkey()],
        )?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.wallet.pubkey()),
            &[&self.wallet],
            self.connection.get_latest_blockhash()?,
        );

        let signature = self.connection.send_and_confirm_transaction(&transaction)?;
        Ok((signature.to_string(), rent_lamports))
    }

    /// -- 关闭单个代币账户的内部实现
    ///
    /// # 参数
    /// * `account_pubkey` - 要关闭的账户公钥
    ///
    /// # 返回
    /// * `Result<(String, u64), Box<dyn Error>>` - 成功返回 (交易签名, 租金金额)，失败返回错误
    async fn close_account_internal(
        &self,
        account_pubkey: &Pubkey,
    ) -> Result<(String, u64), Box<dyn Error>> {
        let details = self.get_account_details(account_pubkey).await?;

        if details.balance != 0 {
            return Err("账户余额不为 0，无法关闭".into());
        }

        self.execute_close_account(account_pubkey, details.rent_lamports)
            .await
    }

    /// -- 关闭单个代币账户
    ///
    /// # 参数
    /// * `account_pubkey` - 要关闭的账户公钥
    ///
    /// # 返回
    /// * `ClosureResult` - 账户关闭操作的结果
    pub async fn close_account(&self, account_pubkey: &Pubkey) -> ClosureResult {
        match self.close_account_internal(account_pubkey).await {
            Ok((signature, rent)) => ClosureResult {
                success: true,
                signature: Some(signature),
                error: None,
                account_address: account_pubkey.to_string(),
                rent_recovered: rent as f64 / LAMPORTS_PER_SOL as f64,
            },
            Err(e) => ClosureResult {
                success: false,
                signature: None,
                error: Some(e.to_string()),
                account_address: account_pubkey.to_string(),
                rent_recovered: 0.0,
            },
        }
    }

    /// -- 创建批量关闭账户的交易
    ///
    /// # 参数
    /// * `accounts` - 要关闭的账户列表
    ///
    /// # 返回
    /// * `Result<(Transaction, f64), Box<dyn Error>>` - 成功返回 (交易对象, 预计回收租金)
    async fn create_batch_close_transaction(
        &self,
        accounts: &[TokenAccountInfo],
    ) -> Result<(Transaction, f64), Box<dyn Error>> {
        let mut instructions = Vec::new();
        let mut total_rent_recovered = 0.0;

        for account in accounts {
            let pubkey = Pubkey::from_str(&account.address)?;
            let instruction = close_account(
                &spl_token::id(),
                &pubkey,
                &self.wallet.pubkey(),
                &self.wallet.pubkey(),
                &[&self.wallet.pubkey()],
            )?;
            instructions.push(instruction);
            total_rent_recovered += account.rent_sol;
        }

        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&self.wallet.pubkey()),
            &[&self.wallet],
            self.connection.get_latest_blockhash()?,
        );

        Ok((transaction, total_rent_recovered))
    }

    /// -- 批量关闭多个代币账户
    ///
    /// # 参数
    /// * `accounts` - 要关闭的账户列表
    /// * `batch_size` - 每批处理的账户数量
    /// * `use_batch_tx` - 是否使用批量交易（true: 合并交易，false: 单独交易）
    pub async fn batch_close_accounts(
        &self,
        accounts: &[TokenAccountInfo],
        batch_size: usize,
        use_batch_tx: bool,
    ) -> Result<(), Box<dyn Error>> {
        if accounts.is_empty() {
            warn!("没有找到可关闭的账户");
            return Ok(());
        }

        let balance_before = self
            .connection
            .get_balance(&self.wallet.pubkey())
            .unwrap_or(0);
        let balance_before_sol = balance_before as f64 / LAMPORTS_PER_SOL as f64;

        let mut total_rent_recovered = 0.0;
        let mut success_count = 0;
        let mut fail_count = 0;

        for (i, chunk) in accounts.chunks(batch_size).enumerate() {
            info!("\n处理第 {} 批, 共 {} 个账户", i + 1, chunk.len());

            if use_batch_tx {
                // -- 批量交易模式
                let (transaction, chunk_rent) = self.create_batch_close_transaction(chunk).await?;
                total_rent_recovered += chunk_rent;

                match self.connection.send_and_confirm_transaction(&transaction) {
                    Ok(signature) => {
                        success_count += chunk.len();
                        info!("批量关闭成功，交易签名: {}", signature);
                        for account in chunk {
                            info!("成功关闭账户: {}", account.address);
                        }
                    }
                    Err(e) => {
                        fail_count += chunk.len();
                        error!("批量关闭失败: {}", e);
                        for account in chunk {
                            error!("账户关闭失败: {}", account.address);
                        }
                    }
                }
            } else {
                // -- 单独交易模式
                for account in chunk {
                    let pubkey = Pubkey::from_str(&account.address).unwrap();
                    let result = self.close_account(&pubkey).await;

                    if result.success {
                        success_count += 1;
                        total_rent_recovered += result.rent_recovered;
                        info!("成功关闭账户: {}", result.account_address);
                        info!("交易签名: {}", result.signature.unwrap());
                        info!("回收租金: {} SOL", result.rent_recovered);
                    } else {
                        fail_count += 1;
                        error!("关闭失败: {}", result.account_address);
                        error!("错误信息: {}", result.error.unwrap());
                    }
                }
            }

            // -- 批次间延时
            if i * batch_size < accounts.len() {
                thread::sleep(Duration::from_millis(2000));
            }
        }

        // -- 统计结果部分保持不变
        let balance_after = self
            .connection
            .get_balance(&self.wallet.pubkey())
            .unwrap_or(0);
        let balance_after_sol = balance_after as f64 / LAMPORTS_PER_SOL as f64;
        let actual_recovered = balance_after_sol - balance_before_sol;
        let gas_consumed = actual_recovered - total_rent_recovered;

        info!("\n====== 处理完成 ======");
        info!("执行前钱包余额: {} SOL", balance_before_sol);
        info!("执行后钱包余额: {} SOL", balance_after_sol);
        info!("实际增加余额: {} SOL", actual_recovered);
        info!("成功关闭: {} 个账户", success_count);
        info!("失败数量: {} 个账户", fail_count);
        info!("预计回收租金: {} SOL", total_rent_recovered);
        info!("GAS 消耗: {} SOL", gas_consumed);

        Ok(())
    }

    /// -- 获取所有可关闭的代币账户列表
    ///
    /// # 返回
    /// * `Result<TokenAccountsResult, Box<dyn Error>>` - 成功返回可关闭账户列表及统计信息，失败返回错误
    pub async fn get_closeable_accounts(&self) -> Result<TokenAccountsResult, Box<dyn Error>> {
        let accounts = self.connection.get_token_accounts_by_owner(
            &self.wallet.pubkey(),
            TokenAccountsFilter::ProgramId(spl_token::id()),
        )?;

        let mut closeable_accounts = Vec::new();
        let mut total_rent_lamports = 0;
        let mut total_rent_sol = 0.0;

        for account in &accounts {
            // -- 解析 JSON 数据
            if let solana_account_decoder::UiAccountData::Json(parsed_data) = &account.account.data
            {
                if let Some(info) = parsed_data.parsed.get("info") {
                    // -- 修改这部分代码，添加错误处理
                    if let Some(mint) = info.get("mint") {
                        let mint_str = mint.to_string();
                        // -- 移除引号
                        let clean_mint = mint_str.trim_matches('"');
                        // -- 使用 match 处理可能的错误
                        match fetch_token_info(&self.connection, clean_mint) {
                            Ok(token_info) => {
                                info!("代币元数据: {}", format_metadata(&token_info.0));
                            }
                            Err(e) => {
                                warn!("获取代币信息失败: {}, 继续处理下一个账户", e);
                                // -- 继续处理，不中断循环
                                continue;
                            }
                        }
                    }
                    if let Some(token_amount) = info.get("tokenAmount") {
                        // -- 获取代币数量
                        let amount = token_amount
                            .get("amount")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(1); // 如果解析失败，默认设为非 0 值

                        // -- 只处理余额为 0 的账户
                        if amount == 0 {
                            let rent_lamports = account.account.lamports;
                            let rent_sol = rent_lamports as f64 / LAMPORTS_PER_SOL as f64;

                            // -- 获取 mint 地址
                            let mint = info
                                .get("mint")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string();

                            total_rent_lamports += rent_lamports;
                            total_rent_sol += rent_sol;

                            closeable_accounts.push(TokenAccountInfo {
                                address: account.pubkey.to_string(),
                                mint,
                                rent_lamports,
                                rent_sol,
                            });
                        }
                    }
                }
            }
        }

        let result = TokenAccountsResult {
            total_accounts: accounts.len(),
            closable_accounts: closeable_accounts.len(),
            accounts: closeable_accounts,
            total_rent_lamports,
            total_rent_sol,
        };

        // -- 打印统计信息
        info!("{}", "=".repeat(50));
        info!("账户统计");
        info!("{}", "=".repeat(50));
        info!("总账户数: {}", result.total_accounts);
        info!("可关闭账户数: {}", result.closable_accounts);
        info!("总可回收租金: {} SOL", result.total_rent_sol);

        // -- 打印详细信息
        if !result.accounts.is_empty() {
            info!("{}", "=".repeat(50));
            info!("可关闭账户详情");
            info!("{}", "=".repeat(50));

            for (index, account) in result.accounts.iter().enumerate() {
                info!("[账户 {}]", index + 1);
                info!("地址: {}", account.address);
                info!("Mint: {}", account.mint);
                info!("租金: {} SOL", account.rent_sol);
            }
        }

        info!("{}", "=".repeat(50));

        Ok(result)
    }

    /// -- 销毁代币并回收账户
    ///
    /// # 参数
    /// * `account_pubkey` - 要操作的账户公钥
    ///
    /// # 返回
    /// * `Result<BurnAndCloseResult, Box<dyn Error>>` - 操作结果
    pub async fn burn_and_close_account(&self, account_pubkey: &Pubkey) -> BurnAndCloseResult {
        let mut result = BurnAndCloseResult {
            success: false,
            burn_signature: None,
            close_signature: None,
            error: None,
            account_address: account_pubkey.to_string(),
            burned_amount: 0,
            rent_recovered: 0.0,
        };

        // -- 获取账户详情
        match self.get_account_details(account_pubkey).await {
            Ok(details) => {
                if details.balance == 0 {
                    // -- 如果余额为 0，直接关闭账户
                    let close_result = self.close_account(account_pubkey).await;
                    result.success = close_result.success;
                    result.close_signature = close_result.signature;
                    result.error = close_result.error;
                    result.rent_recovered = close_result.rent_recovered;
                } else {
                    // -- 1. 销毁代币
                    let mint_pubkey = Pubkey::from_str(&details.mint).unwrap();
                    let burn_instruction = spl_token::instruction::burn(
                        &spl_token::id(),
                        account_pubkey,
                        &mint_pubkey,
                        &self.wallet.pubkey(),
                        &[&self.wallet.pubkey()],
                        details.balance,
                    )
                    .unwrap();

                    let recent_blockhash = self.connection.get_latest_blockhash().unwrap();
                    let burn_tx = Transaction::new_signed_with_payer(
                        &[burn_instruction],
                        Some(&self.wallet.pubkey()),
                        &[&self.wallet],
                        recent_blockhash,
                    );

                    match self.connection.send_and_confirm_transaction(&burn_tx) {
                        Ok(signature) => {
                            result.burn_signature = Some(signature.to_string());
                            result.burned_amount = details.balance;

                            // -- 2. 关闭账户
                            let close_result = self.close_account(account_pubkey).await;
                            result.success = close_result.success;
                            result.close_signature = close_result.signature;
                            result.rent_recovered = close_result.rent_recovered;
                        }
                        Err(e) => {
                            result.error = Some(format!("销毁代币失败: {}", e));
                            return result;
                        }
                    }
                }
            }
            Err(e) => {
                result.error = Some(format!("获取账户详情失败: {}", e));
            }
        }

        result
    }
}
