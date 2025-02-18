use account_info::*;
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
use whitelist::TokenWhitelist;

mod account_info;
mod whitelist;

// TokenAccountManager struct
pub struct TokenAccountManager {
    pub connection: RpcClient,
    pub wallet: Keypair,
    whitelist: TokenWhitelist, // -- 添加白名单字段
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

        let key_str = read_to_string(wallet_key_path)?;
        let key_value: serde_json::Value = serde_json::from_str(&key_str)?;
        let private_key = key_value.as_str().ok_or("Invalid key format")?;

        let wallet = Keypair::from_base58_string(private_key);
        let whitelist = TokenWhitelist::new(); // -- 初始化白名单

        Ok(Self {
            connection,
            wallet,
            whitelist,
        })
    }

    /// -- 添加代币符号到白名单
    pub fn add_symbol_to_whitelist(&mut self, symbol: &str) {
        self.whitelist.add_symbol(symbol);
    }

    /// -- 添加 Mint 地址到白名单
    pub fn add_mint_to_whitelist(&mut self, mint: &str) {
        self.whitelist.add_mint(mint);
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
                            // -- 获取并打印 symbol
                            if let Ok((metadata, _mint)) =
                                fetch_token_info(&self.connection, &account.mint)
                            {
                                let symbol =
                                    metadata.symbol.trim_matches(char::from(0)).to_string();
                                info!("代币地址: {}, Symbol: {}", account.mint, symbol);
                            } else {
                                info!("代币地址: {}, 无法获取 Symbol", account.mint);
                            }
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

                        // -- 获取并打印 symbol
                        if let Ok((metadata, _mint)) =
                            fetch_token_info(&self.connection, &account.mint)
                        {
                            let symbol = metadata.symbol.trim_matches(char::from(0)).to_string();
                            info!("代币地址: {}, Symbol: {}", account.mint, symbol);
                        } else {
                            info!("代币地址: {}, 无法获取 Symbol", account.mint);
                        }

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

    /// -- 获取所有可关闭的代币账户列表（包括零值代币账户）
    ///
    /// # 返回
    /// * `Result<TokenAccountsResult, Box<dyn Error>>` - 成功返回可关闭账户列表及统计信息，失败返回错误
    pub async fn get_closeable_accounts(&self) -> Result<TokenAccountsResult, Box<dyn Error>> {
        let accounts = self.connection.get_token_accounts_by_owner(
            &self.wallet.pubkey(),
            TokenAccountsFilter::ProgramId(spl_token::id()),
        )?;

        let mut closeable_accounts = Vec::new();
        let mut zero_value_accounts = Vec::new();
        let mut total_rent_lamports = 0;
        let mut total_rent_sol = 0.0;

        for account in &accounts {
            if let solana_account_decoder::UiAccountData::Json(parsed_data) = &account.account.data
            {
                if let Some(info) = parsed_data.parsed.get("info") {
                    if let Some(mint) = info.get("mint") {
                        let mint_str = mint.to_string();
                        let clean_mint = mint_str.trim_matches('"');

                        // -- 获取代币信息
                        let token_info = match fetch_token_info(&self.connection, clean_mint) {
                            Ok(token_info) => {
                                info!("代币元数据: {}", format_metadata(&token_info.0));
                                Some(token_info)
                            }
                            Err(e) => {
                                warn!("获取代币信息失败: {}, 继续处理下一个账户", e);
                                None
                            }
                        };

                        if let Some(token_amount) = info.get("tokenAmount") {
                            let amount = token_amount
                                .get("amount")
                                .and_then(|v| v.as_str())
                                .and_then(|s| s.parse::<u64>().ok())
                                .unwrap_or(0);

                            let rent_lamports = account.account.lamports;
                            let rent_sol = rent_lamports as f64 / LAMPORTS_PER_SOL as f64;
                            let mint = clean_mint.to_string();
                            let symbol = token_info
                                .as_ref()
                                .map(|(metadata, _)| {
                                    metadata.symbol.trim_matches(char::from(0)).to_string()
                                })
                                .unwrap_or_else(|| "unknown".to_string());

                            total_rent_lamports += rent_lamports;
                            total_rent_sol += rent_sol;

                            if amount == 0 {
                                // -- 余额为 0 的账户
                                closeable_accounts.push(TokenAccountInfo {
                                    address: account.pubkey.to_string(),
                                    mint: mint.clone(),
                                    rent_lamports,
                                    rent_sol,
                                    symbol: symbol.clone(),
                                });
                            } else {
                                // -- 检查是否为零值代币，且不在白名单中
                                if let Some((metadata, _)) = token_info {
                                    let symbol =
                                        metadata.symbol.trim_matches(char::from(0)).to_string();
                                    if !self.whitelist.is_whitelisted(&symbol, &mint) {
                                        zero_value_accounts.push(ZeroValueTokenInfo {
                                            address: account.pubkey.to_string(),
                                            mint,
                                            balance: amount,
                                            rent_lamports,
                                            rent_sol,
                                            symbol,
                                        });
                                    } else {
                                        info!(
                                            "跳过白名单代币 - Symbol: {}, Mint: {}",
                                            symbol, mint
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let result = TokenAccountsResult {
            total_accounts: accounts.len(),
            closable_accounts: closeable_accounts.len(),
            zero_value_accounts: zero_value_accounts.len(),
            accounts: closeable_accounts,
            zero_value_accounts_list: zero_value_accounts,
            total_rent_lamports,
            total_rent_sol,
        };

        // -- 打印统计信息
        info!("{}", "=".repeat(50));
        info!("账户统计");
        info!("{}", "=".repeat(50));
        info!("总账户数: {}", result.total_accounts);
        info!("可关闭账户数（余额为 0）: {}", result.closable_accounts);
        info!("零值代币账户数: {}", result.zero_value_accounts);
        info!("总可回收租金: {} SOL", result.total_rent_sol);

        // -- 打印详细信息
        if !result.accounts.is_empty() {
            info!("{}", "=".repeat(50));
            info!("可关闭账户详情（余额为 0）");
            info!("{}", "=".repeat(50));

            for (index, account) in result.accounts.iter().enumerate() {
                info!("[账户 {}]", index + 1);
                info!("地址: {}", account.address);
                info!("Mint: {}", account.mint);
                info!("租金: {} SOL", account.rent_sol);
                info!("Symbol: {}", account.symbol);
            }
        }

        if !result.zero_value_accounts_list.is_empty() {
            info!("{}", "=".repeat(50));
            info!("零值代币账户详情（非白名单）");
            info!("{}", "=".repeat(50));

            for (index, account) in result.zero_value_accounts_list.iter().enumerate() {
                info!("[账户 {}]", index + 1);
                info!("地址: {}", account.address);
                info!("Mint: {}", account.mint);
                info!("余额: {}", account.balance);
                info!("租金: {} SOL", account.rent_sol);
                info!("Symbol: {}", account.symbol);
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

    /// -- 批量销毁并关闭零值代币账户
    ///
    /// # 参数
    /// * `accounts` - 要关闭的零值代币账户列表
    /// * `batch_size` - 每批处理的账户数量
    pub async fn batch_burn_and_close_zero_value_accounts(
        &self,
        accounts: &[ZeroValueTokenInfo],
        batch_size: usize,
    ) -> Result<(), Box<dyn Error>> {
        if accounts.is_empty() {
            warn!("没有找到可关闭的零值代币账户");
            return Ok(());
        }

        let balance_before = self
            .connection
            .get_balance(&self.wallet.pubkey())
            .unwrap_or(0);
        let balance_before_sol = balance_before as f64 / LAMPORTS_PER_SOL as f64;

        let mut success_count = 0;
        let mut fail_count = 0;
        let mut total_rent_recovered = 0.0;

        for (i, chunk) in accounts.chunks(batch_size).enumerate() {
            info!("\n处理第 {} 批, 共 {} 个账户", i + 1, chunk.len());

            for account in chunk {
                let pubkey = Pubkey::from_str(&account.address).unwrap();
                let result = self.burn_and_close_account(&pubkey).await;

                if result.success {
                    success_count += 1;
                    total_rent_recovered += result.rent_recovered;

                    info!("成功处理账户: {}", result.account_address);
                    info!("销毁数量: {}", result.burned_amount);
                    info!("销毁交易: {}", result.burn_signature.unwrap());
                    info!("关闭交易: {}", result.close_signature.unwrap());
                    info!("回收租金: {} SOL", result.rent_recovered);
                } else {
                    fail_count += 1;
                    error!("处理失败: {}", result.account_address);
                    error!("错误信息: {}", result.error.unwrap());
                }
            }

            // -- 批次间延时
            if i * batch_size < accounts.len() {
                thread::sleep(Duration::from_millis(2000));
            }
        }

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
        info!("成功处理: {} 个账户", success_count);
        info!("失败数量: {} 个账户", fail_count);
        info!("预计回收租金: {} SOL", total_rent_recovered);
        info!("GAS 消耗: {} SOL", gas_consumed);

        Ok(())
    }
}
