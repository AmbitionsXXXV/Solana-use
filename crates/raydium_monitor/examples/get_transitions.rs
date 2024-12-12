use anyhow::Result;
use raydium_monitor::{
    client::get_transaction_details,
    decoder::decode_ix_data,
    model::{InstructionDataValue, RaydiumInstruction},
    services::process_transaction,
    token_info::fetch_token_info,
    utils::init_tracing,
};
use serde_json::json;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let signature =
        "5bZxPrnrFWj9ebAfVsGbAf35k8s7pKNVdXx2ETzqrJMmWjFnwYk4duFyJvSLC3Hcu39UzV8PNpXwiMoKe8Jbdm6K";
    let tx: solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta =
        get_transaction_details(signature).await?;
    let ray = String::from("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

    // 处理交易数据
    let (instruction_data, _) = process_transaction(&tx, &ray)?;

    match instruction_data.value {
        InstructionDataValue::AccountsAndData { accounts, data } => {
            // 定义相关账户的索引
            let lp_index = 4;
            let token_a_index = 8;
            let token_b_index = 9;

            // 获取相关账户地址
            let lp_account = &accounts[lp_index];
            let token_a_account = &accounts[token_a_index];
            let token_b_account = &accounts[token_b_index];

            // 获取代币 A 和代币 B 的信息
            info!("正在获取代币 A 的信息: {}", token_a_account);
            let token_a = fetch_token_info(token_a_account)?;
            info!("正在获取代币 B 的信息: {}", token_b_account);
            let token_b = fetch_token_info(token_b_account)?;

            let decoded_ix_data = decode_ix_data::<RaydiumInstruction>(&data.unwrap())?;

            info!("新流动性池创建成功!");
            info!("交易链接：https://solscan.io/tx/{}", signature);
            info!("新的 LP 地址：{}", lp_account);

            // 构建显示数据
            let display_data = vec![
                json!({
                    "代币": token_a.0.name.trim_matches(char::from(0)),
                    "账户公钥": token_a_account,
                    "数量": decoded_ix_data.init_coin_amount as f64 / 10f64.powi(token_a.1 as i32),
                    "代币精度": token_a.1,
                }),
                json!({
                    "代币": token_b.0.name.trim_matches(char::from(0)),
                    "账户公钥": token_b_account,
                    "数量": decoded_ix_data.init_pc_amount as f64 / 10f64.powi(token_b.1 as i32),
                    "代币精度": token_b.1,
                }),
            ];

            // 打印流动性池详情
            info!(
                "流动性池详情:\n{}",
                serde_json::to_string_pretty(&display_data)?
            );
        }
        InstructionDataValue::Amount(amount) => {
            println!("Amount: {}", amount);
        }
    }

    Ok(())
}
