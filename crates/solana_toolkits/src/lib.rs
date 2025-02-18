use account_info::*;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, program_pack::Pack,
    pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};
use spl_token::{instruction::close_account, state::Account};
use std::future::Future;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::{error::Error, fs::read_to_string, str::FromStr, thread, time::Duration};
use tracing::{error, info, warn};
use utils::{fetch_token_info, format_metadata, init_rpc_client};
use utils::{TokenAccountError, TokenAccountResult};
use whitelist::TokenWhitelist;

/// -- Solana 代币账户管理工具
///
/// 该模块提供了一系列用于管理 Solana 代币账户的工具，包括：
/// - 查询账户信息
/// - 关闭零余额账户
/// - 批量关闭账户
/// - 白名单管理
/// - 资源回收
pub mod account_info;
pub mod whitelist;

/// -- 代币账户管理配置
///
/// 用于配置代币账户管理器的各项参数
#[derive(Debug, Clone)]
pub struct TokenAccountConfig {
    /// Solana 网络提交配置
    pub commitment: CommitmentConfig,
    /// 批处理操作间隔时间
    pub batch_delay: Duration,
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔时间
    pub retry_delay: Duration,
}

impl Default for TokenAccountConfig {
    fn default() -> Self {
        Self {
            commitment: CommitmentConfig::confirmed(),
            batch_delay: Duration::from_millis(2000),
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
        }
    }
}

/// -- 代币账户管理器
///
/// 提供了一系列方法来管理 Solana 代币账户，包括查询、关闭和批量操作等功能。
/// 支持白名单管理，可以保护特定代币账户不被误关闭。
pub struct TokenAccountManager {
    /// RPC 客户端连接
    pub connection: RpcClient,
    /// 钱包密钥对
    pub wallet: Keypair,
    /// 代币白名单
    whitelist: TokenWhitelist,
    /// 管理器配置
    config: TokenAccountConfig,
}

impl TokenAccountManager {
    /// -- 创建新的代币账户管理器实例
    ///
    /// 使用默认配置创建一个新的代币账户管理器。
    ///
    /// # 参数
    /// * `wallet_key_path` - 钱包密钥文件路径
    ///
    /// # 返回
    /// * `TokenAccountResult<Self>` - 成功返回管理器实例，失败返回错误
    ///
    /// # 示例
    /// ```no_run
    /// use solana_toolkits::TokenAccountManager;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let manager = TokenAccountManager::new("wallet.json")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(wallet_key_path: &str) -> TokenAccountResult<Self> {
        Self::with_config(wallet_key_path, TokenAccountConfig::default())
    }

    /// -- 使用自定义配置创建代币账户管理器实例
    ///
    /// # 参数
    /// * `wallet_key_path` - 钱包密钥文件路径
    /// * `config` - 自定义配置参数
    ///
    /// # 返回
    /// * `TokenAccountResult<Self>` - 成功返回管理器实例，失败返回错误
    pub fn with_config(
        wallet_key_path: &str,
        config: TokenAccountConfig,
    ) -> TokenAccountResult<Self> {
        let connection = init_rpc_client(config.commitment)?;

        let key_str = read_to_string(wallet_key_path)?;
        let key_value: serde_json::Value = serde_json::from_str(&key_str)?;
        let private_key = key_value
            .as_str()
            .ok_or(TokenAccountError::InvalidKeyFormat)?;

        let wallet = Keypair::from_base58_string(private_key);
        let whitelist = TokenWhitelist::new(Some(true));

        Ok(Self {
            connection,
            wallet,
            whitelist,
            config,
        })
    }

    /// -- 获取当前配置
    ///
    /// 返回管理器当前使用的配置参数
    pub fn get_config(&self) -> &TokenAccountConfig {
        &self.config
    }

    /// -- 更新配置
    ///
    /// 更新管理器的配置参数。如果提交配置发生变化，会自动更新 RPC 客户端。
    ///
    /// # 参数
    /// * `config` - 新的配置参数
    ///
    /// # 返回
    /// * `TokenAccountResult<()>` - 成功返回 Ok(()), 失败返回错误
    pub fn update_config(&mut self, config: TokenAccountConfig) -> TokenAccountResult<()> {
        // 如果 commitment 发生变化，需要更新 RPC 客户端
        if self.config.commitment != config.commitment {
            self.connection = init_rpc_client(config.commitment)?;
        }
        self.config = config;
        Ok(())
    }

    /// -- 设置是否合并默认白名单
    ///
    /// 控制是否将用户自定义的白名单与默认白名单（USDC、USDT、SOL）合并
    ///
    /// # 参数
    /// * `merge_default` - true 表示合并，false 表示只使用自定义白名单
    pub fn set_merge_default_whitelist(&mut self, merge_default: bool) {
        self.whitelist.set_merge_default(merge_default);
    }

    /// -- 添加代币符号到白名单
    ///
    /// # 参数
    /// * `symbol` - 代币符号，如 "RAY"、"BONK" 等
    pub fn add_symbol_to_whitelist(&mut self, symbol: &str) {
        self.whitelist.add_symbol(symbol);
    }

    /// -- 批量添加代币符号到白名单
    ///
    /// # 参数
    /// * `symbols` - 代币符号列表
    pub fn add_symbols_to_whitelist(&mut self, symbols: &[&str]) {
        self.whitelist.add_symbols(symbols);
    }

    /// -- 添加 Mint 地址到白名单
    ///
    /// # 参数
    /// * `mint` - 代币的 Mint 地址
    pub fn add_mint_to_whitelist(&mut self, mint: &str) {
        self.whitelist.add_mint(mint);
    }

    /// -- 批量添加 Mint 地址到白名单
    ///
    /// # 参数
    /// * `mints` - Mint 地址列表
    pub fn add_mints_to_whitelist(&mut self, mints: &[&str]) {
        self.whitelist.add_mints(mints);
    }

    /// -- 检查代币是否在白名单中
    ///
    /// # 参数
    /// * `symbol` - 代币符号
    /// * `mint` - 代币的 Mint 地址
    ///
    /// # 返回
    /// * `bool` - true 表示在白名单中，false 表示不在
    pub fn is_token_whitelisted(&self, symbol: &str, mint: &str) -> bool {
        self.whitelist.is_whitelisted(symbol, mint)
    }

    /// -- 获取指定账户的详细信息
    ///
    /// 获取代币账户的详细信息，包括余额、租金等。
    ///
    /// # 参数
    /// * `account_pubkey` - 账户的公钥
    ///
    /// # 返回
    /// * `TokenAccountResult<TokenAccountDetails>` - 成功返回账户详情，失败返回错误
    pub async fn get_account_details(
        &self,
        account_pubkey: &Pubkey,
    ) -> TokenAccountResult<TokenAccountDetails> {
        let account_info = self
            .connection
            .get_account(account_pubkey)
            .map_err(TokenAccountError::from)?;

        let token_account = Account::unpack(&account_info.data)
            .map_err(|e| TokenAccountError::AccountParseError(e.to_string()))?;

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
    /// 关闭指定的代币账户，回收租金。
    ///
    /// # 参数
    /// * `account_pubkey` - 要关闭的账户公钥
    /// * `rent_lamports` - 账户当前的租金金额
    ///
    /// # 返回
    /// * `TokenAccountResult<(String, u64)>` - 成功返回 (交易签名, 租金金额)，失败返回错误
    pub async fn execute_close_account(
        &self,
        account_pubkey: &Pubkey,
        rent_lamports: u64,
    ) -> TokenAccountResult<(String, u64)> {
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
            self.connection
                .get_latest_blockhash()
                .map_err(TokenAccountError::from)?,
        );

        let signature = self
            .connection
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| TokenAccountError::TransactionError(e.to_string()))?;

        Ok((signature.to_string(), rent_lamports))
    }

    /// -- 关闭单个代币账户的内部实现
    ///
    /// 内部使用的账户关闭实现，包含余额检查等安全措施。
    ///
    /// # 参数
    /// * `account_pubkey` - 要关闭的账户公钥
    ///
    /// # 返回
    /// * `TokenAccountResult<(String, u64)>` - 成功返回 (交易签名, 租金金额)，失败返回错误
    async fn close_account_internal(
        &self,
        account_pubkey: &Pubkey,
    ) -> TokenAccountResult<(String, u64)> {
        let details = self.get_account_details(account_pubkey).await?;

        if details.balance != 0 {
            return Err(TokenAccountError::NonZeroBalance(details.balance));
        }

        self.execute_close_account(account_pubkey, details.rent_lamports)
            .await
    }

    /// -- 关闭单个代币账户
    ///
    /// 对外暴露的账户关闭接口，提供友好的结果格式。
    ///
    /// # 参数
    /// * `account_pubkey` - 要关闭的账户公钥
    ///
    /// # 返回
    /// * `ClosureResult` - 包含操作结果的详细信息
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

    /// -- 创建批量关闭交易
    ///
    /// 为多个账户创建一个批量关闭交易。
    ///
    /// # 参数
    /// * `accounts` - 要关闭的账户列表
    ///
    /// # 返回
    /// * `TokenAccountResult<(Transaction, f64)>` - 成功返回 (交易对象, 预计回收租金)
    async fn create_batch_close_transaction(
        &self,
        accounts: &[TokenAccountInfo],
    ) -> TokenAccountResult<(Transaction, f64)> {
        let mut instructions = Vec::new();
        let mut total_rent_recovered = 0.0;

        for account in accounts {
            let pubkey = Pubkey::from_str(&account.address)
                .map_err(|e| TokenAccountError::AccountParseError(e.to_string()))?;
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

    /// -- 通用批量处理函数
    ///
    /// 提供带重试机制的批量处理功能。
    ///
    /// # 参数
    /// * `items` - 要处理的项目列表
    /// * `batch_size` - 每批处理的数量
    /// * `process_fn` - 处理函数
    ///
    /// # 返回
    /// * `TokenAccountResult<()>` - 处理结果
    async fn process_batch_with_retry<'a, T, F, Fut>(
        &'a self,
        items: &'a [T],
        batch_size: usize,
        process_fn: F,
    ) -> TokenAccountResult<()>
    where
        F: Fn(&'a [T]) -> Fut + Send + 'a,
        Fut: Future<Output = TokenAccountResult<()>> + Send + 'a,
        T: Sync + 'a,
    {
        for (i, chunk) in items.chunks(batch_size).enumerate() {
            info!("\n处理第 {} 批, 共 {} 个项目", i + 1, chunk.len());

            let mut retries = 0;
            loop {
                match process_fn(chunk).await {
                    Ok(_) => break,
                    Err(e) if retries < self.config.max_retries => {
                        retries += 1;
                        warn!("重试第 {} 次: {}", retries, e);
                        tokio::time::sleep(self.config.retry_delay).await;
                    }
                    Err(e) => return Err(e),
                }
            }
            tokio::time::sleep(self.config.batch_delay).await;
        }
        Ok(())
    }

    /// -- 批量关闭账户
    ///
    /// 批量关闭多个代币账户，支持单独交易和批量交易两种模式。
    ///
    /// # 参数
    /// * `accounts` - 要关闭的账户列表
    /// * `batch_size` - 每批处理的账户数量
    /// * `use_batch_tx` - 是否使用批量交易（true: 合并交易，false: 单独交易）
    ///
    /// # 返回
    /// * `TokenAccountResult<()>` - 处理结果
    pub async fn batch_close_accounts(
        &self,
        accounts: &[TokenAccountInfo],
        batch_size: usize,
        use_batch_tx: bool,
    ) -> TokenAccountResult<()> {
        if accounts.is_empty() {
            warn!("没有找到可关闭的账户");
            return Ok(());
        }

        let balance_before = self.connection.get_balance(&self.wallet.pubkey())?;
        let balance_before_sol = balance_before as f64 / LAMPORTS_PER_SOL as f64;

        let success_count = Arc::new(AtomicUsize::new(0));
        let fail_count = Arc::new(AtomicUsize::new(0));
        let total_rent_recovered = Arc::new(AtomicU64::new(0));

        if use_batch_tx {
            // -- 批量交易模式
            let success_count_clone = Arc::clone(&success_count);
            let fail_count_clone = Arc::clone(&fail_count);
            let total_rent_recovered_clone = Arc::clone(&total_rent_recovered);

            self.process_batch_with_retry(accounts, batch_size, move |chunk| {
                let success_count = Arc::clone(&success_count_clone);
                let fail_count = Arc::clone(&fail_count_clone);
                let total_rent_recovered = Arc::clone(&total_rent_recovered_clone);

                async move {
                    let (transaction, chunk_rent) =
                        self.create_batch_close_transaction(chunk).await?;
                    let rent_lamports = (chunk_rent * LAMPORTS_PER_SOL as f64) as u64;
                    total_rent_recovered.fetch_add(rent_lamports, Ordering::SeqCst);

                    match self.connection.send_and_confirm_transaction(&transaction) {
                        Ok(signature) => {
                            success_count.fetch_add(chunk.len(), Ordering::SeqCst);
                            info!("批量关闭成功，交易签名: {}", signature);
                            for account in chunk {
                                if let Ok((metadata, _)) =
                                    fetch_token_info(&self.connection, &account.mint)
                                        .map_err(|e| TokenAccountError::Other(e.to_string()))
                                {
                                    let symbol =
                                        metadata.symbol.trim_matches(char::from(0)).to_string();
                                    info!("代币地址: {}, Symbol: {}", account.mint, symbol);
                                }
                                info!("成功关闭账户: {}", account.address);
                            }
                            Ok(())
                        }
                        Err(e) => {
                            fail_count.fetch_add(chunk.len(), Ordering::SeqCst);
                            Err(TokenAccountError::TransactionError(e.to_string()))
                        }
                    }
                }
            })
            .await?;
        } else {
            // -- 单独交易模式
            let success_count_clone = Arc::clone(&success_count);
            let fail_count_clone = Arc::clone(&fail_count);
            let total_rent_recovered_clone = Arc::clone(&total_rent_recovered);

            self.process_batch_with_retry(accounts, batch_size, move |chunk| {
                let success_count = Arc::clone(&success_count_clone);
                let fail_count = Arc::clone(&fail_count_clone);
                let total_rent_recovered = Arc::clone(&total_rent_recovered_clone);

                async move {
                    for account in chunk {
                        let pubkey = Pubkey::from_str(&account.address)
                            .map_err(|e| TokenAccountError::AccountParseError(e.to_string()))?;

                        match self.close_account_internal(&pubkey).await {
                            Ok((signature, rent_lamports)) => {
                                success_count.fetch_add(1, Ordering::SeqCst);
                                total_rent_recovered.fetch_add(rent_lamports, Ordering::SeqCst);

                                if let Ok((metadata, _)) =
                                    fetch_token_info(&self.connection, &account.mint)
                                        .map_err(|e| TokenAccountError::Other(e.to_string()))
                                {
                                    let symbol =
                                        metadata.symbol.trim_matches(char::from(0)).to_string();
                                    info!("代币地址: {}, Symbol: {}", account.mint, symbol);
                                }

                                info!("成功关闭账户: {}", account.address);
                                info!("交易签名: {}", signature);
                                info!(
                                    "回收租金: {} SOL",
                                    rent_lamports as f64 / LAMPORTS_PER_SOL as f64
                                );
                            }
                            Err(e) => {
                                fail_count.fetch_add(1, Ordering::SeqCst);
                                error!("关闭失败: {}", account.address);
                                error!("错误信息: {}", e);
                            }
                        }
                    }
                    Ok(())
                }
            })
            .await?;
        }

        // -- 统计结果
        let balance_after = self.connection.get_balance(&self.wallet.pubkey())?;
        let balance_after_sol = balance_after as f64 / LAMPORTS_PER_SOL as f64;
        let actual_recovered = balance_after_sol - balance_before_sol;
        let total_rent_recovered_sol =
            total_rent_recovered.load(Ordering::SeqCst) as f64 / LAMPORTS_PER_SOL as f64;
        let gas_consumed = actual_recovered - total_rent_recovered_sol;

        info!("\n====== 处理完成 ======");
        info!("执行前钱包余额: {} SOL", balance_before_sol);
        info!("执行后钱包余额: {} SOL", balance_after_sol);
        info!("实际增加余额: {} SOL", actual_recovered);
        info!("成功关闭: {} 个账户", success_count.load(Ordering::SeqCst));
        info!("失败数量: {} 个账户", fail_count.load(Ordering::SeqCst));
        info!("预计回收租金: {} SOL", total_rent_recovered_sol);
        info!("GAS 消耗: {} SOL", gas_consumed);

        Ok(())
    }

    /// -- 获取可关闭的代币账户列表
    ///
    /// 获取所有可以关闭的代币账户，包括：
    /// - 余额为 0 的账户
    /// - 不在白名单中的零值代币账户
    ///
    /// # 返回
    /// * `TokenAccountResult<TokenAccountsResult>` - 包含可关闭账户列表和统计信息
    pub async fn get_closeable_accounts(&self) -> TokenAccountResult<TokenAccountsResult> {
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
                                    if !self.is_token_whitelisted(&symbol, &mint) {
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
    /// 销毁账户中的代币并关闭账户，回收租金。
    /// 如果账户余额为 0，则直接关闭账户。
    ///
    /// # 参数
    /// * `account_pubkey` - 要操作的账户公钥
    ///
    /// # 返回
    /// * `BurnAndCloseResult` - 包含操作结果的详细信息：
    ///   - 销毁交易签名
    ///   - 关闭交易签名
    ///   - 销毁的代币数量
    ///   - 回收的租金数量
    ///   - 错误信息（如果有）
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
    /// 批量处理零值代币账户，包括：
    /// 1. 销毁账户中的代币
    /// 2. 关闭账户并回收租金
    ///
    /// # 参数
    /// * `accounts` - 要处理的零值代币账户列表
    /// * `batch_size` - 每批处理的账户数量
    ///
    /// # 返回
    /// * `Result<(), Box<dyn Error>>` - 处理结果
    ///
    /// # 说明
    /// - 每个账户都会单独处理，确保操作的安全性
    /// - 会自动跳过白名单中的代币账户
    /// - 处理过程中会记录详细的操作日志
    /// - 最后会输出统计信息，包括：
    ///   - 成功处理的账户数量
    ///   - 失败的账户数量
    ///   - 回收的总租金
    ///   - GAS 消耗
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
