use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StudentIntroError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("Input data exceeds max length")]
    InvalidDataLength,
}

impl From<StudentIntroError> for ProgramError {
    fn from(e: StudentIntroError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
