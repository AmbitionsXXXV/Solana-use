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
    manager.get_closeable_accounts().await?;

    Ok(())
}
