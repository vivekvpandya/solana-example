use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::FundError::InvalidInstruction;

pub enum FundInstruction {
    /// Invest in the fund.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the invester.
    /// 1. `[writable]` The invester's USDC account.
    /// 2. `[writable]` The invester's fund token account.
    /// 3. `[writable]` The fund's USDC account.
    /// 4. `[writable]` The fund's fund token account.
    /// 5. `[]` The fund's primary account.
    /// 6. `[]` The rent sysvar.
    /// 7. `[]` The token program.
    Invest {
        /// The amount of USDC invester wants to invest.
        amount: u64,
    },
}

impl FundInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Invest {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
