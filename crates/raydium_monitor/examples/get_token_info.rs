use anyhow::Result;
use solana_sdk::commitment_config::CommitmentConfig;
use tracing::info;
use utils::{fetch_token_info, init_rpc_client, init_tracing, load_env};

fn main() -> Result<()> {
    init_tracing();
    load_env()?;

    let rpc_client = init_rpc_client(CommitmentConfig::confirmed())?;
    let (metadata, mint) =
        fetch_token_info(&rpc_client, "5KJPXhymz4pv2gpNcTsFquCp57v3b4QBhDa1zQcnpump")?;

    info!("Token metadata: {:#?}", metadata);
    info!("Token decimals: {}", mint.decimals);

    Ok(())
}
