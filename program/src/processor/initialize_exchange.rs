use solana_program::{pubkey::Pubkey, rent::Rent, account_info::{AccountInfo, next_account_info}, msg, program::invoke_signed, system_instruction::create_account, entrypoint::ProgramResult, sysvar::Sysvar};
use borsh::BorshSerialize;
use crate::errors::TokenTracingError;
use crate::state::*;
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let acounts_iter = &mut accounts.iter();
    let payer = next_account_info(acounts_iter)?;
    let vault = next_account_info(acounts_iter)?;
    let program = next_account_info(acounts_iter)?;
    let mint = next_account_info(acounts_iter)?;
    let (vault_pda, vault_bump_seed) = Pubkey::find_program_address(&[b"vault", mint.key.as_ref()], program_id);
    msg!("{} | {}", vault_pda, *vault.key );
    if vault_pda != *vault.key {
        msg!("Invalid account key for vault");
        return Err(TokenTracingError::InvalidVaultAccount.into());
    }

    msg!("create vault {} ...", vault.key.to_string());
    invoke_signed(
        &create_account(
            &payer.key,
            &vault.key,
            Rent::get()?.minimum_balance(EXCHANGE_ACCOUNT_LEN),
            EXCHANGE_ACCOUNT_LEN as u64,
            program_id,
        ),
        &[payer.clone(), program.clone(), vault.clone()],
        &[&[b"vault", mint.key.as_ref(), &[vault_bump_seed]]],
    )?;
    msg!("=================================");
    // * Allocate data to vault
    let account_info = ExchangeAccount {
        admin: *payer.key,
        vault: *vault.key
    };


    let account_data = &mut *vault.data.borrow_mut();
    account_info.serialize(account_data)?;

    Ok(())
}
