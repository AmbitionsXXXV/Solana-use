use solana_transaction_status::UiInnerInstructions;

use anyhow::Result;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::client::{get_transaction_details, init_rpc_client};
use crate::decoder::decode_instruction_data;
use crate::model::InstructionDataValue;
use crate::services::{process_instruction, process_transaction};
use crate::token_info::get_token_addresses;
use crate::utils::log_swap_operation;

/// 分析交换信息
///
/// 该函数分析给定交易签名的交换操作，并输出相关信息。
///
/// # 参数
///
/// * `signature` - 交易签名字符串
///
/// # 返回值
///
/// 返回 `Result<()>`
pub async fn analyze_swap_info(signature: String) -> Result<()> {
    // 步骤 1：创建 RPC 客户端
    let rpc_client = init_rpc_client(CommitmentConfig::confirmed())?;

    // 步骤 2：获取交易详情
    let tx = get_transaction_details(&signature).await?;
    let ray = String::from("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
    let (instruction_data, inner_ixs) = process_transaction(&tx, &ray)?;

    // 步骤 3：处理指令数据
    match instruction_data.value {
        InstructionDataValue::AccountsAndData { accounts, data } => {
            // 解码指令数据
            let decoded_data = decode_instruction_data(&data)?;

            // 步骤 4：获取代币账户信息
            let (source_address, dest_address) =
                get_token_addresses(&rpc_client, &accounts).await?;

            // 步骤 6：根据代币地址判断操作类型并记录日志
            log_swap_operation(
                accounts,
                source_address,
                dest_address,
                decoded_data,
                inner_ixs,
            )?;
        }
        InstructionDataValue::Amount(_) => {}
    }

    Ok(())
}

/// 获取实际交换数量
///
/// # 参数
///
/// * `decimals` - 代币小数位数
/// * `inner_ixs` - 内部指令
///
/// # 返回值
///
/// 返回实际交换数量
pub fn get_actual_amount(decimals: u8, inner_ixs: Option<UiInnerInstructions>) -> u64 {
    if inner_ixs.is_none() {
        return 0;
    }

    let parsed_ix = process_instruction(&inner_ixs.unwrap().instructions[1], "")
        .unwrap()
        .value;

    match parsed_ix {
        InstructionDataValue::AccountsAndData {
            accounts: _,
            data: _,
        } => 0,
        InstructionDataValue::Amount(amount) => amount / 10u64.pow(decimals as u32),
    }
}

/// 计算滑点
///
/// # 参数
///
/// * `actual` - 实际数量
/// * `expected` - 预期数量
///
/// # 返回值
///
/// 返回滑点百分比
pub fn calculate_slippage(actual: f64, expected: f64) -> f64 {
    (actual - expected) / expected * 100.00
}
