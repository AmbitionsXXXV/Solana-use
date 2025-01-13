use std::env;

use anyhow::Result;
use raydium_monitor::{services::subscribe_to_logs, utils::load_env};
use tracing::info;
use utils::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    load_env()?;
    init_tracing();

    let ws_url = env::var("HELIUS_WS_RPC_URL")?;
    info!("Helius WS RPC URL: {}", ws_url);
    subscribe_to_logs(&ws_url).await?;

    Ok(())
}
