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
    let manager = TokenAccountManager::new(&wallet_path)?;

    // -- 获取可关闭账户信息
    let accounts = manager.get_closeable_accounts().await?;

    // -- 批量关闭账户，每批处理 5 个
    manager
        .batch_close_accounts(&accounts.accounts, 1, false)
        .await?;

    Ok(())
}
