use anyhow::Result;
use raydium_monitor::{swap_analyzer::analyze_swap_info, utils::init_tracing};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    analyze_swap_info(
        "3VAxEp6xee6AdufMqCeYz6b2xJmNHH6kWMJhNcjgGZStgiDHsiXEm7UiGek954wjhh9r416Dyxt8xJL1C4piYGbo"
            .to_string(),
    )
    .await?;

    Ok(())
}
