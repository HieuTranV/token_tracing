use crate::errors::TokenTracingError;
use crate::instruction::TokenTracingInstruction;

use solana_program::{
    account_info::AccountInfo,
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    msg,
    program_error::{PrintProgramError, ProgramError},
    pubkey::Pubkey,
};


mod initialize_exchange;
mod exchange_SOL_to_token;
mod exchange_token_to_SOL;
// pub mod deposit;
// pub mod exchange;
// pub mod initialize_exchange_booth;
// pub mod utils;
// pub mod withdraw;
pub struct Processor;

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("xbooth: process instructions");
        let instruction = TokenTracingInstruction::unpack(instruction_data).map_err(|err| {
            msg!("invalid instruction data. cause {:}", err);
            ProgramError::InvalidInstructionData
        })?;
        msg!("instruction: {:?}", instruction);
        match instruction {
            TokenTracingInstruction::Initialize {} => {
                msg!("Initialize");
                initialize_exchange::process(&program_id, &accounts)?;
            },
            TokenTracingInstruction::ExchangeSOLToToken { amount } => {
                msg!("Exchange SOL to token");
                exchange_SOL_to_token::process(program_id, accounts, amount);
                // deposit::process(program_id, accounts, amount)?;
            },
            TokenTracingInstruction::ExchangeTokenToSOL { amount } => {
                msg!("Exchange token to SOL");
                exchange_token_to_SOL::process(program_id, accounts, amount);
                // withdraw::process(program_id, accounts, amount)?;
            }
    
        }
        Ok(())
    }
}

impl PrintProgramError for TokenTracingError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
