use anyhow::Result;
use serde_json::{json, Value};
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiInnerInstructions,
    UiInstruction, UiMessage, UiParsedInstruction,
};
use tracing::{debug, error, info, instrument, warn};

use crate::client::get_transaction_details;
use crate::decoder::decode_ix_data;
use crate::model::{InstructionData, InstructionDataValue, MonitorError, RaydiumInstruction};
use crate::token_info::fetch_token_info;

/// 订阅并处理 Solana 日志
///
/// 该函数连接到指定的 WebSocket URL，订阅特定程序 ID 的日志，
/// 并处理与新建流动性池相关的交易。
///
/// # 参数
///
/// * `ws_url` - WebSocket URL 字符串
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作成功或失败
#[instrument]
pub async fn subscribe_to_logs(ws_url: &str) -> Result<()> {
    info!("正在订阅日志");
    // 步骤 1：连接 WebSocket 并订阅特定程序 ID 的日志
    let (_, logs_receiver) = PubsubClient::logs_subscribe(
        ws_url,
        RpcTransactionLogsFilter::Mentions(vec![
            "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()
        ]),
        RpcTransactionLogsConfig {
            commitment: Some(CommitmentConfig::confirmed()),
        },
    )?;

    info!("成功订阅日志");

    // 步骤 2：持续处理接收到的日志
    loop {
        match logs_receiver.recv() {
            Ok(response) => {
                debug!("收到日志响应");
                // 步骤 3：检查是否为 initialize2 指令的日志
                if response.value.err.is_none()
                    && response
                        .value
                        .logs
                        .iter()
                        .any(|log| log.contains("initialize2"))
                {
                    let signature = response.value.signature;
                    info!("正在处理交易，签名: {}", signature);

                    // 步骤 4：获取交易详情
                    let tx = get_transaction_details(&signature).await?;

                    let ray = String::from("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

                    // 步骤 5：处理交易数据
                    let (instruction_data, _) = process_transaction(&tx, &ray)?;

                    // 步骤 6：根据指令数据类型进行处理
                    match instruction_data.value {
                        InstructionDataValue::AccountsAndData { accounts, data } => {
                            // 获取相关账户地址
                            let lp_account = &accounts[4];
                            let token_a_account = &accounts[8];
                            let token_b_account = &accounts[9];

                            // 步骤 7：获取代币信息
                            info!("正在获取代币 A 的信息: {}", token_a_account);
                            let token_a = fetch_token_info(token_a_account)?;
                            info!("正在获取代币 B 的信息: {}", token_b_account);
                            let token_b = fetch_token_info(token_b_account)?;

                            // 步骤 8：解码指令数据
                            let decoded_ix_data =
                                decode_ix_data::<RaydiumInstruction>(&data.unwrap())?;

                            // 步骤 9：打印新流动性池信息
                            info!("新流动性池创建成功!");
                            info!("交易链接：https://solscan.io/tx/{}", signature);
                            info!("新的 LP 地址：{}", lp_account);

                            // 步骤 10：构建并打印显示数据
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

                            info!(
                                "流动性池详情:\n{}",
                                serde_json::to_string_pretty(&display_data)?
                            );

                            info!("交易处理成功");
                        }
                        InstructionDataValue::Amount(amount) => {
                            println!("Amount: {}", amount);
                        }
                    }
                }
            }
            Err(e) => {
                error!("账户订阅错误: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

/// 处理交易数据，提取指定程序 ID 的指令信息
///
/// 该函数解析交易数据，寻找与目标程序 ID 匹配的指令，并返回相关的指令数据。
///
/// # 参数
///
/// * `tx` - 编码后的确认交易及其元数据
/// * `target_program_id` - 目标程序 ID 字符串
///
/// # 返回值
///
/// 返回 `Result<InstructionData, MonitorError>`，包含匹配指令的数据，
/// 或在未找到匹配指令或遇到其他错误时返回 `MonitorError`。
#[instrument(skip(tx), fields(target_program_id = %target_program_id))]
pub fn process_transaction(
    tx: &EncodedConfirmedTransactionWithStatusMeta,
    target_program_id: &str,
) -> Result<(InstructionData, Option<UiInnerInstructions>), MonitorError> {
    info!("开始处理交易");
    match &tx.transaction.transaction {
        EncodedTransaction::Json(t) => match &t.message {
            UiMessage::Raw(message) => {
                // 步骤 1：查找目标程序 ID 的索引
                let ray_index = message
                    .account_keys
                    .iter()
                    .position(|key| key == target_program_id)
                    .ok_or_else(|| {
                        error!("未找到目标程序 ID");
                        MonitorError::ProgramIdNotFound
                    })?;

                info!(ray_index, "找到目标程序索引");

                // 步骤 2：返回所有账户信息
                Ok((
                    InstructionData {
                        value: InstructionDataValue::AccountsAndData {
                            accounts: message.account_keys.clone(),
                            data: None,
                        },
                    },
                    None,
                ))
            }
            UiMessage::Parsed(message) => {
                debug!("处理已解析的消息");
                // 步骤 3：处理指令并返回第一个匹配的 InstructionData 和相应的 InnerInstruction
                message
                    .instructions
                    .iter()
                    .enumerate()
                    .find_map(|(index, ix)| {
                        let data = process_instruction(ix, target_program_id);
                        if data.is_some() {
                            info!(instruction_index = index, "找到匹配的指令");
                        }
                        // 步骤 4：查找对应的内部指令
                        if let Some(meta) = &tx.transaction.meta {
                            match &meta.inner_instructions {
                                OptionSerializer::Some(inner_instructions) => {
                                    let inner_instruction = inner_instructions
                                        .iter()
                                        .find(|inner_instruction| {
                                            inner_instruction.index == index as u8
                                        })
                                        .cloned();

                                    if let Some(d) = data {
                                        return Some((d, inner_instruction));
                                    }
                                }
                                _ => {
                                    warn!("未找到内部指令");
                                }
                            }
                        }

                        data.map(|d| (d, None))
                    })
                    .ok_or(MonitorError::NoMatchingInstruction)
            }
        },
        _ => {
            error!("不支持的交易格式");
            Err(MonitorError::UnsupportedTransactionFormat)
        }
    }
}

/// 处理单个指令，提取与目标程序 ID 匹配的指令数据
///
/// # 参数
///
/// * `instruction` - UI 指令对象
/// * `target_program_id` - 目标程序 ID 字符串
///
/// # 返回值
///
/// 返回 `Option<InstructionData>`，如果找到匹配的指令则返回 Some(InstructionData)，
/// 否则返回 None。
#[instrument(skip(instruction), fields(target_program_id = %target_program_id))]
pub fn process_instruction(
    instruction: &UiInstruction,
    target_program_id: &str,
) -> Option<InstructionData> {
    match instruction {
        UiInstruction::Compiled(compiled_instruction) => {
            debug!("处理已编译的指令");
            // 步骤 1：处理已编译的指令
            Some(InstructionData {
                value: InstructionDataValue::AccountsAndData {
                    accounts: compiled_instruction
                        .accounts
                        .iter()
                        .map(|&i| i.to_string())
                        .collect(),
                    data: Some(compiled_instruction.data.clone()),
                },
            })
        }
        UiInstruction::Parsed(parsed_instruction) => match parsed_instruction {
            UiParsedInstruction::Parsed(parsed_ix) => {
                debug!("处理完全解析的指令");
                // 步骤 2：处理完全解析的指令
                match &parsed_ix.parsed {
                    Value::Object(map) => {
                        if let Some(nested_obj) = map.get("info").and_then(|v| v.as_object()) {
                            if let Some(inner_value) =
                                nested_obj.get("amount").and_then(|v| v.as_str())
                            {
                                if let Ok(amount) = inner_value.parse::<u64>() {
                                    debug!("解析到金额: {}", amount);

                                    return Some(InstructionData {
                                        value: InstructionDataValue::Amount(amount),
                                    });
                                }
                            }
                        }
                        None
                    }
                    _ => None,
                }
            }
            UiParsedInstruction::PartiallyDecoded(partially_decoded_instruction) => {
                // 步骤 3：处理部分解码的指令
                if partially_decoded_instruction.program_id == target_program_id {
                    info!("找到匹配的部分解码指令");
                    Some(InstructionData {
                        value: InstructionDataValue::AccountsAndData {
                            accounts: partially_decoded_instruction.accounts.clone(),
                            data: Some(partially_decoded_instruction.data.clone()),
                        },
                    })
                } else {
                    debug!("部分解码指令不匹配目标程序 ID");
                    None
                }
            }
        },
    }
}
