use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use solana_program::msg;
use crate::errors::TokenTracingError;
#[derive(Debug)]
pub enum TokenTracingInstruction {
    Initialize,
    ExchangeSOLToToken { amount: u32 },
    ExchangeTokenToSOL { amount: u32 }
}

impl TokenTracingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("+++++++++++++++++++++++++++++++++++++");
        
        let (&tag, rest) = input.split_first().ok_or(TokenTracingError::InvalidInstruction)?;
        msg!("--------------------------------------------");
        return match tag {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::ExchangeSOLToToken { amount: Self::get_amount(rest)?}),
            2 => Ok(Self::ExchangeTokenToSOL { amount: Self::get_amount(rest)?}),
            _ => Err(TokenTracingError::InvalidInstructionData.into())
        };
    }

    fn get_amount(rest: &[u8]) -> Result<u32, ProgramError> {
        let raw_data: Result<[u8; 4], _> = rest[..4].try_into();
        match raw_data {
            Ok(i) => Ok(u32::from_le_bytes(i)),
            
            _ => Err(TokenTracingError::InvalidInstructionData.into())
        }

    }
}

