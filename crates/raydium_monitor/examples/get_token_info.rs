use anyhow::Result;
use raydium_monitor::token_info::fetch_token_info;

fn main() -> Result<()> {
    let (metadata, decimals) = fetch_token_info("5KJPXhymz4pv2gpNcTsFquCp57v3b4QBhDa1zQcnpump")?;

    println!("Token metadata: {:#?}", metadata);
    println!("Token decimals: {}", decimals);

    Ok(())
}
