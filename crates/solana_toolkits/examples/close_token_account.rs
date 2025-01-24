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

    // -- 获取要关闭的账户地址
    let account_pubkey = pubkey!("7Bv7ME271LLxpDRdmVmYbMHkc9ngrHrKMyPkNXpFZeKs");

    // -- 先获取账户详情，验证账户是否有效
    match manager.get_account_details(&account_pubkey).await {
        Ok(details) => {
            info!("账户详情:");
            info!("Mint: {}", details.mint);
            info!("余额: {}", details.balance);

            // -- 检查余额是否为 0
            if details.balance > 0 {
                error!("账户余额不为 0，无法关闭账户");
                return Ok(());
            }

            // -- 执行关闭操作
            let result = manager.close_account(&account_pubkey).await;

            if result.success {
                info!("成功关闭账户: {}", result.account_address);
                info!("交易签名: {}", result.signature.unwrap());
                info!("回收租金: {} SOL", result.rent_recovered);
            } else {
                error!("关闭失败: {}", result.account_address);
                error!("错误信息: {}", result.error.unwrap());
            }
        }
        Err(e) => {
            error!("获取账户详情失败: {}", e);
        }
    }

    Ok(())
}
