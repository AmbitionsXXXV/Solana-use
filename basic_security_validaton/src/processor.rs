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

pub fn my_try_from_slice_unchecked<T: borsh::BorshDeserialize>(
    data: &[u8],
) -> Result<T, ProgramError> {
    let mut data_mut = data;

    match T::deserialize(&mut data_mut) {
        Ok(result) => Ok(result),
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = IntroInstruction::unpack(instruction_data)?;
    match instruction {
        IntroInstruction::InitUserInput { name, message } => {
            add_student_intro(program_id, accounts, name, message)
        }
        IntroInstruction::UpdateStudentIntro { name, message } => {
            update_student_intro(program_id, accounts, name, message)
        }
    }
}

pub fn add_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    msg!("Adding student intro...");
    msg!("Name: {}", name);
    msg!("Message: {}", message);

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);

    if pda != *user_account.key {
        msg!("Invalid seeds for PDA");
        return Err(StudentIntroError::InvalidPDA.into());
    }

    let total_len: usize = 1 + (4 + name.len()) + (4 + message.len());
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(StudentIntroError::InvalidDataLength.into());
    }
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

    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        my_try_from_slice_unchecked::<StudentInfo>(&user_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    msg!("checking if account is already initialized");
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.name = name;
    account_data.msg = message;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

pub fn update_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    msg!("Updating student intro...");
    msg!("Name: {}", name);
    msg!("Message: {}", message);

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;

    msg!("unpacking state account");
    let mut account_data =
        my_try_from_slice_unchecked::<StudentInfo>(&user_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    msg!("checking if account is initialized");
    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(StudentIntroError::UninitializedAccount.into());
    }

    if user_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let (pda, _bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);

    if pda != *user_account.key {
        msg!("Invalid seeds for PDA");
        return Err(StudentIntroError::InvalidPDA.into());
    }

    let update_len: usize = 1 + (4 + account_data.name.len()) + (4 + message.len());
    if update_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(StudentIntroError::InvalidDataLength.into());
    }

    account_data.msg = message;

    msg!("serializing account");
    account_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
