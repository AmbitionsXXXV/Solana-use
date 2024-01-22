// 引入模块和库
use crate::error::StudentIntroError;
use crate::instruction::IntroInstruction;
use crate::state::StudentInfo;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

// 自定义的反序列化函数，用于将字节数组转换为特定的数据类型 T。
pub fn my_try_from_slice_unchecked<T: borsh::BorshDeserialize>(
    data: &[u8],
) -> Result<T, ProgramError> {
    let mut data_mut = data;

    match T::deserialize(&mut data_mut) {
        Ok(result) => Ok(result),
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}

// 智能合约的主处理函数，用于解析和执行传入的指令。
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 解析指令数据为 IntroInstruction 类型。
    let instruction = IntroInstruction::unpack(instruction_data)?;

    // 根据指令类型调用相应的处理函数。
    match instruction {
        IntroInstruction::InitUserInput { name, message } => {
            add_student_intro(program_id, accounts, name, message)
        }
        IntroInstruction::UpdateStudentIntro { name, message } => {
            update_student_intro(program_id, accounts, name, message)
        }
    }
}

// 添加学生介绍信息的处理函数。
pub fn add_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    // 日志输出
    msg!("Adding student intro...");
    msg!("Name: {}", name);
    msg!("Message: {}", message);

    // 获取账户迭代器
    let account_info_iter = &mut accounts.iter();

    // 解析账户信息
    let initializer = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // 计算 PDA (Program Derived Address)
    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);

    // 验证 PDA
    if pda != *user_account.key {
        msg!("Invalid seeds for PDA");

        return Err(StudentIntroError::InvalidPDA.into());
    }

    // 计算数据长度，验证是否超出限制
    let total_len: usize = 1 + (4 + name.len()) + (4 + message.len());

    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");

        return Err(StudentIntroError::InvalidDataLength.into());
    }

    // 创建账户并分配存储空间
    let account_len: usize = 1000;
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            user_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            user_account.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), &[bump_seed]]],
    )?;

    // 反序列化并更新账户数据
    msg!("unpacking state account");

    let mut account_data =
        my_try_from_slice_unchecked::<StudentInfo>(&user_account.data.borrow()).unwrap();

    if account_data.is_initialized() {
        msg!("Account already initialized");

        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.name = name;
    account_data.msg = message;
    account_data.is_initialized = true;

    // 序列化并保存账户数据
    msg!("serializing account");
    account_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

// 更新学生介绍信息的处理函数。
pub fn update_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    // 日志输出
    msg!("Updating student intro...");
    msg!("Name: {}", name);
    msg!("Message: {}", message);

    // 获取账户迭代器
    let account_info_iter = &mut accounts.iter();

    // 解析账户信息
    let initializer = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;

    // 反序列化账户数据
    msg!("unpacking state account");
    let mut account_data =
        my_try_from_slice_unchecked::<StudentInfo>(&user_account.data.borrow()).unwrap();

    // 验证账户状态
    msg!("checking if account is initialized");
    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(StudentIntroError::UninitializedAccount.into());
    }
    if user_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    // 计算 PDA
    let (pda, _bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);

    // 验证 PDA
    if pda != *user_account.key {
        msg!("Invalid seeds for PDA");
        return Err(StudentIntroError::InvalidPDA.into());
    }

    // 更新数据并验证长度
    let update_len: usize = 1 + (4 + account_data.name.len()) + (4 + message.len());

    if update_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(StudentIntroError::InvalidDataLength.into());
    }
    account_data.msg = message;

    // 序列化并保存账户数据
    msg!("serializing account");
    account_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
