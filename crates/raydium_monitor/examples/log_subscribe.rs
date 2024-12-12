use std::env;

use anyhow::Result;
use raydium_monitor::{services::subscribe_to_logs, utils::load_env};

#[tokio::main]
async fn main() -> Result<()> {
    load_env()?;

    let ws_url = env::var("SOLANA_WS_RPC_URL")?;
    subscribe_to_logs(&ws_url).await?;

    Ok(())
}
