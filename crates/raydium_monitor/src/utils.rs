use std::env;
use std::path::Path;

use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiInnerInstructions;
use tracing::info;

use crate::model::SwapIxData;
use crate::swap_analyzer::{calculate_slippage, get_actual_amount};
use crate::token_info::fetch_token_info;

/// 初始化 tracing 日志系统
///
/// 该函数设置和初始化 tracing 日志系统，使用环境变量配置的过滤器。
pub fn init_tracing() {
  use tracing_subscriber::{fmt, EnvFilter};

  // 使用环境变量配置的过滤器初始化 tracing
  fmt().with_env_filter(EnvFilter::from_default_env()).init();
}

pub fn load_env() -> Result<()> {
  let manifest_dir =
    env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
  let workspace_root = Path::new(&manifest_dir)
    .parent()
    .and_then(Path::parent)
    .expect("Failed to find workspace root");
  let dot_env_path = workspace_root.join(".env");

  dotenv::from_path(&dot_env_path)?;

  Ok(())
}

pub fn log_swap_operation(
  accounts: Vec<String>,
  source_address: Option<Pubkey>,
  dest_address: Option<Pubkey>,
  decoded_data: Option<SwapIxData>,
  inner_ixs: Option<UiInnerInstructions>,
) -> Result<()> {
  match (source_address, dest_address, decoded_data) {
    (Some(source), Some(dest), Some(decoded)) => {
      log_sell_operation(accounts, source, dest, decoded, inner_ixs)
    }
    (None, Some(dest), Some(decoded)) => {
      log_buy_operation(accounts, dest, decoded, inner_ixs)
    }
    _ => Ok(()),
  }
}

/// 记录买入操作日志
///
/// # 参数
///
/// * `accounts` - 账户列表
/// * `destination_token_address` - 目标代币地址
/// * `decoded_ix` - 解码后的指令数据
/// * `inner_ix` - 内部指令
///
/// # 返回值
///
/// 返回 `Result<()>`
pub fn log_buy_operation(
  accounts: Vec<String>,
  destination_token_address: Pubkey,
  decoded_ix: SwapIxData,
  inner_ix: Option<UiInnerInstructions>,
) -> Result<()> {
  let token_info = fetch_token_info(destination_token_address)?;
  let actual_amount = get_actual_amount(token_info.1, inner_ix);
  let slippage_rate = calculate_slippage(
    actual_amount as f64,
    decoded_ix.minimum_amount_out as f64 / 10f64.powi(token_info.1 as i32),
  );

  info!(
    "代币全称: {}",
    token_info.0.name.trim_matches(char::from(0))
  );
  info!("代币简称: {}", token_info.0.symbol);
  info!("操作地址：{}", accounts[17]);
  info!(
    "预期花费: {} Sol",
    decoded_ix.amount_in as f64 / 10f64.powi(9)
  );
  info!(
    "预期最少获得: {} {}",
    decoded_ix.minimum_amount_out as f64 / 10f64.powi(token_info.1 as i32),
    token_info.0.symbol
  );
  info!("实际获得: {} {}", actual_amount as f64, token_info.0.symbol);
  info!("滑点: {:.2}%", slippage_rate);

  Ok(())
}

/// 记录卖出操作日志
///
/// # 参数
///
/// * `accounts` - 账户列表
/// * `source_token_address` - 源代币地址
/// * `destination_token_address` - 目标代币地址
/// * `decoded_ix` - 解码后的指令数据
/// * `inner_ix` - 内部指令
///
/// # 返回值
///
/// 返回 `Result<()>`
pub fn log_sell_operation(
  accounts: Vec<String>,
  source_token_address: Pubkey,
  destination_token_address: Pubkey,
  decoded_ix: SwapIxData,
  inner_ix: Option<UiInnerInstructions>,
) -> Result<()> {
  let source_token_info = fetch_token_info(source_token_address)?;
  let destination_token_info = fetch_token_info(destination_token_address)?;
  let actual_amount = get_actual_amount(destination_token_info.1, inner_ix);
  let slippage_rate = calculate_slippage(
    actual_amount as f64,
    decoded_ix.minimum_amount_out as f64 / 10f64.powi(9),
  );

  info!("正在处理 Sell 操作");
  info!(
    "卖出代币: {}",
    source_token_info.0.name.trim_matches(char::from(0))
  );
  info!("操作地址：{}", accounts[17]);
  info!(
    "卖出数量: {} {}",
    decoded_ix.amount_in as f64 / 10f64.powi(source_token_info.1 as i32),
    source_token_info.0.symbol
  );
  info!(
    "预期获得: {} Sol",
    decoded_ix.minimum_amount_out as f64 / 10f64.powi(9),
  );
  info!("实际获得: {} Sol", actual_amount as f64);
  info!("滑点: {:.2}%", slippage_rate);

  Ok(())
}
