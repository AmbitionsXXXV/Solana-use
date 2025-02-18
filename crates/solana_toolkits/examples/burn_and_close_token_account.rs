use anyhow::Result;
use solana_sdk::pubkey;
use solana_toolkits::TokenAccountManager;
use std::error::Error;
use tracing::{error, info};
use utils::{init_tracing, load_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // -- 初始化日志和环境变量
    init_tracing();
    load_env()?;

    // -- 初始化账户管理器
    let wallet_path = std::env::var("WALLET_PATH")?;
    let manager = TokenAccountManager::new(&wallet_path)?;

    let account_pubkey = pubkey!("BftUiGB2iDkNDa8AKdhtLuJHHXiUfqLvTNmiXaSopump");

    // -- 获取要关闭的账户地址
    let result = manager.burn_and_close_account(&account_pubkey).await;

    if result.success {
        info!("代币销毁数量: {}", result.burned_amount);
        info!("回收租金: {} SOL", result.rent_recovered);
        info!("销毁交易签名: {}", result.burn_signature.unwrap());
        info!("关闭交易签名: {}", result.close_signature.unwrap());
    } else {
        error!("操作失败: {}", result.error.unwrap());
    }

    Ok(())
}
