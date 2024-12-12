use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::borsh1::try_from_slice_unchecked;

use crate::model::SwapIxData;

/// 解码 Raydium 指令数据
///
/// 该函数将 base58 编码的指令数据解码并反序列化为 RaydiumInstruction 结构体。
///
/// # 参数
///
/// * `data` - base58 编码的指令数据字符串
///
/// # 返回值
///
/// 返回 `Result<RaydiumInstruction>`，包含解码后的 Raydium 指令数据
pub fn decode_ix_data<T>(data: &str) -> Result<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    // 将 base58 编码的数据解码为字节数组
    let data_slice = bs58::decode(data).into_vec()?;
    // 反序列化为指定的结构体类型
    let d = try_from_slice_unchecked::<T>(&data_slice)?;

    Ok(d)
}

pub fn decode_instruction_data(data: &Option<String>) -> Result<Option<SwapIxData>> {
    data.as_ref()
        .map(|d| decode_ix_data::<SwapIxData>(d))
        .transpose()
}
