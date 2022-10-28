use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use spl_associated_token_account::solana_program::system_instruction;

use crate::errors::TokenTracingError;
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u32) -> ProgramResult {
    msg!("swap sol to token, lamports: {}", amount);
    let accounts_iter = &mut accounts.iter();
    let _program = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let payer_token_account = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let vault_token_account = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (vault_pda, vault_bump_seed) =
        Pubkey::find_program_address(&[b"vault", mint.key.as_ref()], program_id);
    if vault_pda != *vault.key {
        msg!("Invalid vault account");
        return Err(TokenTracingError::InvalidAccountAddress.into());
    }

    msg!("transfer SOL from payer to program");
    
    let pay_sol = system_instruction::transfer(payer.key, vault.key, amount.into());
    let pay_sol_transation_account = [system_program.clone(), payer.clone(), vault.clone()];
    invoke(&pay_sol, 
        &pay_sol_transation_account);

    //send token
    msg!("transfer token from vault_ata: {} to payer_ata: {}", vault_token_account.key.to_string(), payer_token_account.key.to_string());
    let take_token = spl_token::instruction::transfer(
        token_program_id.key,
        &vault_token_account.key,
        &payer_token_account.key,
        &vault.key,
        &[],
        ((amount as u64) *( 10 as u64) as u64),
     )?;
    
    invoke_signed(
        &take_token,
        &[
            token_program_id.clone(),
            payer_token_account.clone(),
            vault_token_account.clone(),
            vault.clone(),
        ],
        &[&[b"vault", mint.key.as_ref(), &[vault_bump_seed]]],
    )?;

    Ok(())
}
