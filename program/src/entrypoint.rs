use crate::errors::TokenTracingError;
use crate::processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::PrintProgramError,
    pubkey::Pubkey,
};

use solana_program::entrypoint;

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        error.print::<TokenTracingError>();
        return Err(error);
    }
    Ok(())
}
