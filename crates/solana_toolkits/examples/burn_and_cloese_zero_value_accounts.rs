use anyhow::Result;
use solana_toolkits::TokenAccountManager;
use std::error::Error;
use utils::{init_tracing, load_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // -- 初始化日志和环境变量
    init_tracing();
    load_env()?;

    // -- 初始化账户管理器
    let wallet_path = std::env::var("WALLET_PATH")?;
    let mut manager = TokenAccountManager::new(&wallet_path)?;

    // -- 添加需要不需要销毁的的 Token symbol
    manager.add_symbol_to_whitelist("JUP");
    manager.add_symbol_to_whitelist("JLP");
    manager.add_symbol_to_whitelist("SUPA");
    manager.add_symbol_to_whitelist("SOL");
    manager.add_symbol_to_whitelist("USDT");
    manager.add_symbol_to_whitelist("USDC");
    manager.add_symbol_to_whitelist("RAY");

    // -- 获取可关闭账户信息
    let res = manager.get_closeable_accounts().await?;

    // -- 批量销毁并关闭数量非零，价值归零的 Token 账户
    manager
        .batch_burn_and_close_zero_value_accounts(&res.zero_value_accounts_list, 10)
        .await?;

    Ok(())
}
