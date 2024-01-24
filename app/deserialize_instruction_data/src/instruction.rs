// 引入 borsh 库的反序列化功能，用于将二进制数据转换成结构体实例。
use borsh::BorshDeserialize;
// 引入 solana_program 库的错误处理模块。
use solana_program::program_error::ProgramError;

// 定义一个名为 MovieInstruction 的枚举类型，用于表示不同的电影评论相关指令。
pub enum MovieInstruction {
    // 一个枚举变量，表示添加电影评论的指令，包含电影的标题、评分和描述。
    AddMovieReview {
        title: String,       // 电影标题
        rating: u8,          // 电影评分，使用u8类型，表示一个0到255的整数
        description: String, // 电影的描述
    },
}

// 使用 BorshDeserialize 特性定义一个结构体，用于反序列化电影评论的数据。
#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,       // 电影标题
    rating: u8,          // 电影评分
    description: String, // 电影描述
}

// 为 MovieInstruction 枚举实现一些功能。
impl MovieInstruction {
    // 定义一个函数，用于解析传入的字节数据，将其转换为MovieInstruction枚举的一个实例。
    // 该函数接收字节数据的引用作为输入，返回一个Result类型，成功时包含MovieInstruction实例，失败时包含ProgramError。
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // 分离输入数据的第一个字节和剩余部分。
        // 第一个字节用于确定指令类型，剩余部分包含具体的指令数据。
        let (&variant, rest) = input
            // 使用split_first方法分割输入数据。
            // split_first返回一个元组，其中包含切片的第一个元素和剩余部分。
            // 如果切片为空，则返回None。
            .split_first()
            // 使用ok_or将Option转换为Result。
            // 如果split_first返回None（即输入数据为空或不正确），
            // ok_or将返回一个包含ProgramError::InvalidInstructionData的Err。
            .ok_or(ProgramError::InvalidInstructionData)?;

        // 使用Borsh反序列化功能将剩余的字节数据转换为 MovieReviewPayload 结构体实例。
        let payload = MovieReviewPayload::try_from_slice(rest).unwrap();

        // 使用匹配表达式根据第一个字节的值创建 MovieInstruction 枚举的不同实例。
        Ok(match variant {
            0 => Self::AddMovieReview {
                title: payload.title,             // 设置电影标题
                rating: payload.rating,           // 设置电影评分
                description: payload.description, // 设置电影描述
            },
            // 如果第一个字节的值无法匹配任何已知指令，返回一个表示无效指令数据的错误。
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
