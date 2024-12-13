use anyhow::Result;
use solana_sdk::commitment_config::CommitmentConfig;
use utils::{fetch_token_info, init_rpc_client};

fn main() -> Result<()> {
    let rpc_client = init_rpc_client(CommitmentConfig::confirmed())?;
    let (metadata, mint) =
        fetch_token_info(&rpc_client, "5KJPXhymz4pv2gpNcTsFquCp57v3b4QBhDa1zQcnpump")?;

    println!("Token metadata: {:#?}", metadata);
    println!("Token decimals: {}", mint.decimals);

    Ok(())
}
