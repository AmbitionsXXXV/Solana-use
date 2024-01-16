use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

// 定义电影评论指令的枚举
pub enum MovieInstruction {
    // 添加电影评论指令，包含标题、评分和描述
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
}

// 用于反序列化的结构体，包含电影评论所需的数据
#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String,
}

impl MovieInstruction {
    // 解析传入的字节缓冲区为相关指令
    // 输入格式预期为 Borsh 序列化的向量
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // 将数据的第一个字节分离
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // 使用 BorshDeserialize 特性的 `try_from_slice` 方法
        // 将指令字节数据反序列化为 payload 结构体
        let payload = MovieReviewPayload::try_from_slice(rest).unwrap();

        // 根据第一个字节的值，匹配并返回 AddMovieReview 结构体
        Ok(match variant {
            0 => Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            // 如果匹配不到，则返回无效指令数据的错误
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
