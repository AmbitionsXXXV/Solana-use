// 引入当前 crate 的 processor 模块。
// 这通常包含了处理指令的具体逻辑。
use crate::processor;

// 引入 solana_program 库中的多个模块。
// 这些模块提供了创建 Solana 智能合约所需的基础功能。
use solana_program::{
    // AccountInfo 用于封装关于账户的信息。
    account_info::AccountInfo,
    // 提供 entrypoint 宏，用于定义智能合约的入口点。
    entrypoint,
    // ProgramResult 是一个 Result 类型，用于表示操作的成功或失败。
    entrypoint::ProgramResult,
    // msg 宏用于在合约中打印消息，有助于调试。
    msg,
    // Pubkey 用于表示公钥。
    pubkey::Pubkey,
};

// 使用 entrypoint 宏定义智能合约的入口点函数。
// 这是合约接收和处理指令的地方。
entrypoint!(process_instruction);

// 定义处理指令的函数。
// 这个函数会被网络节点调用，以执行合约逻辑。
fn process_instruction(
    // program_id 参数表示当前程序的公钥。
    program_id: &Pubkey,
    // accounts 参数包含了与此指令相关的账户信息。
    accounts: &[AccountInfo],
    // instruction_data 参数包含了传给合约的原始指令数据。
    instruction_data: &[u8],
) -> ProgramResult {
    // 使用 msg 宏打印日志信息。
    // 这对调试和追踪合约执行过程很有帮助。
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    // 调用 processor 模块中的 process_instruction 函数来处理指令。
    // 这里使用了 `?` 操作符，它会在发生错误时提前返回。
    processor::process_instruction(program_id, accounts, instruction_data)?;

    // 如果一切顺利，返回 Ok(()) 表示指令处理成功。
    Ok(())
}
