// 引入名为 MovieInstruction 的结构体定义，这个结构体是用来描述影评的指令。
use instruction::MovieInstruction;

// 引入Solana程序开发的相关模块，这些模块提供了账户信息、入口点定义、程序执行结果等功能。
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// 引入当前模块的指令定义，具体内容在instruction模块中定义。
pub mod instruction;

// 设置入口点函数为 process_instruction，这是Solana智能合约的主要入口。
entrypoint!(process_instruction);

// 定义主要处理函数，这个函数会根据不同的指令执行不同的操作。
pub fn process_instruction(
    program_id: &Pubkey,      // 程序的公钥标识
    accounts: &[AccountInfo], // 与程序交互的账户信息列表
    instruction_data: &[u8],  // 指令数据，包含了要执行什么操作的信息
) -> ProgramResult {
    // 返回执行结果，成功或失败
    // 解析指令数据，将其转换为 MovieInstruction 结构体
    let instruction = MovieInstruction::unpack(instruction_data)?;

    // 根据指令类型，执行相应的操作
    match instruction {
        // 如果是添加电影评价的指令
        MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => add_movie_review(program_id, accounts, title, rating, description), // 调用添加电影评价的函数
    }
}

// 定义一个添加电影评价的函数
pub fn add_movie_review(
    program_id: &Pubkey,      // 程序的公钥标识
    accounts: &[AccountInfo], // 与程序交互的账户信息列表
    title: String,            // 电影标题
    rating: u8,               // 电影评分
    description: String,      // 电影描述
) -> ProgramResult {
    // 返回执行结果，成功或失败
    // 记录一些信息，用于调试和追踪
    msg!("Adding movie review..."); // 显示添加电影评价的信息
    msg!("Title: {}", title); // 显示电影标题
    msg!("Rating: {}", rating); // 显示电影评分
    msg!("Description: {}", description); // 显示电影描述
    msg!("Program ID: {}", program_id); // 显示程序ID
    msg!("Accounts: {:?}", accounts); // 显示账户信息

    Ok(()) // 返回成功的执行结果
}
