use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program::invoke, pubkey::Pubkey,
};
use spl_associated_token_account::solana_program::system_instruction;

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u32) -> ProgramResult {
    msg!("swap sol to token, lamports: {}", amount);
    let accounts_iter = &mut accounts.iter();
    let program = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let payer_token_account = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let vault_token_account = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let (vault_pda, vault_bump_seed) = Pubkey::find_program_address(&[b"vault", mint.key.as_ref()], program_id);
    
    msg!("transfer {} Token lamports from payer to vault", amount);
    let send_token = spl_token::instruction::transfer(
        &token_program_id.key,
        &payer_token_account.key,
        &vault_token_account.key,
        &payer.key,
        &[],
        (amount as u64),
    )?;
    invoke(
        &send_token,
        &[
            token_program_id.clone(),
            payer_token_account.clone(),
            vault_token_account.clone(),
            program.clone(),
            payer.clone(),
        ],
    )?;


    **vault.try_borrow_mut_lamports()? -= (amount as u64) / 10 as u64;
    **payer.try_borrow_mut_lamports()? += (amount as u64) / 10 as u64;
    msg!(
        "vault send {} SOL lamports to payer",
        amount / 10 as u32,
    );

    Ok(())
}
