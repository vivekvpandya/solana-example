use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum FundError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Account is not writeable
    #[error("Account is not writeable")]
    AccountIsNotWriteable,
    /// Only USDC is accepted as investment for fund.
    #[error("Only USDC is accepted as investment for fund.")]
    OnlyUSDCAllowed,
    /// Incorrect token address provided.
    #[error("Incorrect token address provided")]
    IncorrectTokenAddress,
}

impl From<FundError> for ProgramError {
    fn from(e: FundError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
