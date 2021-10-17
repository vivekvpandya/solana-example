use std::str::FromStr;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::FundError, instruction::FundInstruction};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = FundInstruction::unpack(instruction_data)?;
        match instruction {
            FundInstruction::Invest { amount } => {
                msg!("Instruction: Invest");
                Self::process_invest_fund(accounts, amount, program_id)
            }
        }
    }

    fn check_usdc_account_constraints(account: &AccountInfo) -> ProgramResult {
        // USDC account must be writable
        if !account.is_writable {
            return Err(FundError::AccountIsNotWriteable.into());
        }
        // USDC is a spl-token
        if account.owner != &spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        // check that it is indeed USDC account
        // USDC testnet id CpMah17kQEL2wqyMKt3mZBdTnZbkbfx4nqmQMFDP5vwp
        // USDC mainnet id EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
        let usdc_pubkey = Pubkey::from_str("CpMah17kQEL2wqyMKt3mZBdTnZbkbfx4nqmQMFDP5vwp")
            .expect("pubkey parsing error");
        if account.key != &usdc_pubkey {
            return Err(FundError::OnlyUSDCAllowed.into());
        }
        // TODO: check for rent exeption
        Ok(())
    }

    fn check_fund_token_account_constraints(account: &AccountInfo) -> ProgramResult {
        // token account must be writable
        if !account.is_writable {
            return Err(FundError::AccountIsNotWriteable.into());
        }
        // token is a spl-token
        if account.owner != &spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        // check that it is indeed fund's token account
        // TODO: devnet token to be created.
        let token_pubkey = Pubkey::from_str("CpMah17kQEL2wqyMKt3mZBdTnZbkbfx4nqmQMFDP5vwp")
            .expect("pubkey parsing error");
        if account.key != &token_pubkey {
            return Err(FundError::IncorrectTokenAddress.into());
        }
        // TODO: check for rent exeption
        Ok(())
    }

    fn process_invest_fund(
        accounts: &[AccountInfo],
        amount: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let investor = next_account_info(account_info_iter)?;

        if !investor.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let investors_usdc_account = next_account_info(account_info_iter)?;
        Self::check_usdc_account_constraints(investors_usdc_account)?;

        let investors_token_account = next_account_info(account_info_iter)?;
        Self::check_fund_token_account_constraints(investors_token_account)?;

        let funds_usdc_account = next_account_info(account_info_iter)?;
        Self::check_usdc_account_constraints(funds_usdc_account)?;

        let funds_token_account = next_account_info(account_info_iter)?;
        Self::check_fund_token_account_constraints(funds_token_account)?;

        let funds_primary_account = next_account_info(account_info_iter)?;
        let _rent_sysvar = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        let transfer_usdc_ix = spl_token::instruction::transfer(
            /*token_program_id*/ token_program.key,
            /*source_pubkey*/ investors_usdc_account.key,
            /*destination_pubkey*/ funds_usdc_account.key,
            /*authority_pubkey*/ investor.key,
            /*signer_pubkeys*/ &[investor.key],
            /*amount*/ amount,
        )?;
        msg!("Calling the token program to transfer USDC tokens from investor to fund");
        invoke(
            &transfer_usdc_ix,
            &[
                investors_usdc_account.clone(),
                funds_usdc_account.clone(),
                investor.clone(),
                token_program.clone(),
            ],
        )?;

        // NOTE: Consider this as new fund offer, so 1 USDC is 1 fund token.
        let transfer_fund_token_ix = spl_token::instruction::transfer(
            /*token_program_id*/ token_program.key,
            /*source_pubkey*/ funds_token_account.key,
            /*destination_pubkey*/ investors_token_account.key,
            /*authority_pubkey*/ funds_primary_account.key,
            /*signer_pubkeys*/ &[funds_primary_account.key],
            /*amount*/ amount,
        )?;
        // TODO: for now program assumes tokens are pre-minted.
        msg!("Calling the token program to transfer fund tokens from fund to investor");
        invoke(
            &transfer_fund_token_ix,
            &[
                funds_token_account.clone(),
                investors_token_account.clone(),
                funds_primary_account.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }
}
