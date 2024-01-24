// 引入 solana_program 库中的 ProgramError 结构体。
// ProgramError 是 Solana 智能合约中用来表示各种程序错误的通用类型。
use solana_program::program_error::ProgramError;

// 引入 thiserror 库的 Error trait。
// 这个库提供了一个方便的方式来定义和处理错误。
use thiserror::Error;

// 定义一个名为 StudentIntroError 的枚举，用于表示可能发生的错误类型。
// 这个枚举会被用于智能合约中以表达特定的错误情况。
#[derive(Debug, Error)]
pub enum StudentIntroError {
    // 表示账户尚未初始化的错误。
    // 当尝试操作一个尚未初始化的账户时会遇到这种错误。
    #[error("Account not initialized yet")]
    UninitializedAccount,

    // 表示提供的 PDA（Program-Derived Address）与期望的 PDA 不符的错误。
    // 在处理与 PDA 相关的操作时，如果计算出的 PDA 与提供的 PDA 不一致，会触发此错误。
    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    // 表示输入数据长度超过了最大限制的错误。
    // 当用户提供的数据长度超出了智能合约处理的范围时，会遇到这种错误。
    #[error("Input data exceeds max length")]
    InvalidDataLength,
}

// 为 StudentIntroError 实现 From trait，使其可以被转换为 ProgramError。
// 这是将自定义错误与 Solana 程序中使用的标准错误类型 ProgramError 联系起来的关键部分。
impl From<StudentIntroError> for ProgramError {
    // 定义转换函数。
    fn from(e: StudentIntroError) -> Self {
        // 将 StudentIntroError 枚举的实例转换为 ProgramError，其中自定义错误的枚举值作为 ProgramError 的自定义代码。
        ProgramError::Custom(e as u32)
    }
}
