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
    let _mint = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let vault_token_account = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // let vault_data = spl_token::state::Account::unpack(&vault.data.borrow())?;
    // if &vault_data.mint != mint.key {
    //     msg!("Invalid mint token");
    //     return Err(TokenSwapError::InvalidMint.into());
    // }

    // msg!("transfer {} Token lamports from payer to vault", amount);
    // let send_token = spl_token::instruction::transfer(
    //     &token_program_id.key,
    //     &payer_token_account.key,
    //     &vault_token_account.key,
    //     &payer.key,
    //     &[],
    //     amount.into(),
    // )?;
    // invoke(
    //     &send_token,
    //     &[
    //         token_program_id.clone(),
    //         payer_token_account.clone(),
    //         vault_token_account.clone(),
    //         program.clone(),
    //         payer.clone(),
    //     ],
    // )?;


// let take_sol = system_instruction::transfer(
//     vault.key, 
//     payer.key, 
//     (amount/10).into(),
// );
// let pay_sol_transation_account = [system_program.clone(), payer.clone(), vault.clone()];
// invoke(&take_sol, 
//     &pay_sol_transation_account);


    msg!("transfer SOL from vault to payer");
    //  **vault.try_borrow_mut_lamports()? -= (amount as u64) / 10 as u64;
    //  **payer.try_borrow_mut_lamports()? += (amount as u64) / 10 as u64;
    msg!(
        "{} SOL lamports transferred from vault to payer",
        amount / 10 as u32,
    );

    Ok(())
}
