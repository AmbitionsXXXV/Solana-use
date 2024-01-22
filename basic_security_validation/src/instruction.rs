// 引入 BorshDeserialize 以支持反序列化操作，这是将二进制数据转换回 Rust 数据结构的过程。
use borsh::BorshDeserialize;
// 引入 ProgramError，用于处理智能合约中的错误情况。
use solana_program::program_error::ProgramError;

// 定义一个枚举 IntroInstruction，用于表示智能合约可以接收的不同类型的指令。
pub enum IntroInstruction {
    // 初始化用户输入的指令，包含用户名和消息。
    InitUserInput { name: String, message: String },
    // 更新学生介绍的指令，也包含用户名和消息。
    UpdateStudentIntro { name: String, message: String },
}

// 定义一个结构体 StudentIntroPayload，用于反序列化传入的指令数据。
// 这个结构体包含了用户的名字和消息，与 IntroInstruction 枚举中的字段相匹配。
#[derive(BorshDeserialize, Debug)]
struct StudentIntroPayload {
    name: String,
    message: String,
}

// 为 IntroInstruction 枚举实现 unpack 方法，用于从原始字节数据中提取指令。
impl IntroInstruction {
    // unpack 方法接收一个字节数组并尝试将其转换为 IntroInstruction 枚举的一个变量。
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // 尝试将输入数据的第一个字节（表示指令类型的变量）和剩余部分分开。
        let (variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // 使用 Borsh 反序列化来解析剩余部分的数据为 StudentIntroPayload。
        let payload = StudentIntroPayload::try_from_slice(rest).unwrap();

        // 根据 variant 的值来确定指令类型，并构造相应的 IntroInstruction 枚举变量。
        Ok(match variant {
            // 如果 variant 是 0，则表示这是一个 InitUserInput 指令。
            0 => Self::InitUserInput {
                name: payload.name,
                message: payload.message,
            },
            // 如果 variant 是 1，则表示这是一个 UpdateStudentIntro 指令。
            1 => Self::UpdateStudentIntro {
                name: payload.name,
                message: payload.message,
            },
            // 如果 variant 是其他值，则表示指令无效。
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
