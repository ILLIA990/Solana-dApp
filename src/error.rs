use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError
{
    #[error("Invalid Instruction")]
    InvalidInstruction,
    
    #[error("Invalid signer")]
    InvalidSigner,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid user storage PDA")]
    InvalidUserStoragePda,

    #[error("Invalid SystemProgram account")]
    SystemProgramMismatch,

    #[error("Account is not initialized")]
    NotInitialized,
}

impl From<StakingError> for ProgramError
{
    fn from(e: StakingError) -> Self
    {
        ProgramError::Custom(e as u32)
    }
}
