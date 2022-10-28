use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, program_error::ProgramError,
};
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, Copy, FromPrimitive, PartialEq)]
pub enum TokenTracingError {
    #[error("Invalid Instruction.")]
    InvalidInstruction,
    #[error("Invalid Instruction Data.")]
    InvalidInstructionData,
    #[error("Invalid Account address.")]
    InvalidAccountAddress,
    #[error("Invalid Vault Account")]
    InvalidVaultAccount,
    #[error("Exchange booth is not writable")]
    ExchangeBoothNotWritable,
    #[error("Account is not writable")]
    AccountIsNotWritable,
    #[error("Account is not signer")]
    AccountIsNotSigner,
    #[error("Not correct owner")]
    InvalidOwner,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account is not initialized")]
    AccountNotInitialized,
    #[error("Invalid SPL token account")]
    InvalidSPLTokenAccount,
    #[error("Invalid mint key")]
    InvalidMint,
    #[error("Accounts cannot have the same mint")]
    UniqueMintAccounts,
}

impl From<TokenTracingError> for ProgramError {
    fn from(e: TokenTracingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TokenTracingError {
    fn type_of() -> &'static str {
        "Exchange booth error"
    }
}
