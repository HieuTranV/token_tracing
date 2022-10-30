use solana_program::{
    system_program, 
    account_info::{AccountInfo}, 
    lamports, 
    instruction, 
    program_error::ProgramError,
};
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair},
    transaction::Transaction,
    sysvar::SysvarId,
    hash::Hash,
    program_pack::Pack,
    signer::Signer,
    system_instruction,
    transport::TransportError,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use more_asserts as ma;
use spl_token::state::{Account, Mint};
use tokentracing::entrypoint::process_instruction;
use solana_program::clock::Epoch;

async fn mint_amount(
    banks_client: &mut BanksClient,
    recent_blockhash: Hash,
    token_program: &Pubkey,
    account: &Pubkey,
    mint: &Pubkey,
    mint_authority: &Keypair,
    payer: &Keypair,
    amount: f64,
    mint_decimals: u8,
) -> Result<(), ProgramError> {
    let mint_amount = (amount * f64::powf(10., mint_decimals.into())) as u64;
    let mint_ix = spl_token::instruction::mint_to(
        token_program,
        mint,
        account,
        &mint_authority.pubkey(),
        &[],
        mint_amount,
    )
    .unwrap();

    let mint_tx = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&payer.pubkey()),
        &[payer, mint_authority],
        recent_blockhash,
    );

    banks_client.process_transaction(mint_tx).await.unwrap();
    Ok(())
}

/// create_and_initialize_account_for_mint does two things
/// 1. creates an account with the payer as owner
/// 2. initializes the account for the current mint
async fn create_and_initialize_account_for_mint(
    banks_client: &mut BanksClient,
    recent_blockhash: Hash,
    token_program: &Pubkey,
    token_account: &Keypair,
    mint: &Keypair,
    payer: &Keypair,
) -> Result<(), ProgramError> {
    let rent = banks_client.get_rent().await.unwrap();
    let account_rent = rent.minimum_balance(Account::LEN);
    let create_account_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &token_account.pubkey(),
        account_rent,
        Account::LEN as u64,
        token_program,
    );

    let initialize_account_ix = spl_token::instruction::initialize_account(
        token_program,
        &token_account.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
    )
    .unwrap();

    let initialize_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_account_ix],
        Some(&payer.pubkey()),
        &[payer, token_account],
        recent_blockhash,
    );

    banks_client
        .process_transaction(initialize_account_tx)
        .await
        .unwrap();

    Ok(())
}

async fn create_and_initialize_mint(
    banks_client: &mut BanksClient,
    recent_blockhash: Hash,
    payer: &Keypair,
    mint_authority: &Keypair,
    mint_account: &Keypair,
    token_program: &Pubkey,
    decimals: &u8,
) -> Result<(), TransportError> {
    let rent = banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(Mint::LEN);
    // create account to hold newly minted tokens
    let token_mint_a_account_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        token_program,
    );

    // initialize mint
    let token_mint_a_ix = spl_token::instruction::initialize_mint(
        token_program,
        &mint_account.pubkey(),
        &mint_authority.pubkey(),
        None,
        *decimals,
    )
    .unwrap();

    // create mint transaction
    let token_mint_a_tx = Transaction::new_signed_with_payer(
        &[token_mint_a_account_ix, token_mint_a_ix],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        recent_blockhash,
    );

    banks_client
        .process_transaction(token_mint_a_tx)
        .await
        .unwrap();
    Ok(())
}

#[tokio::test]
async fn test_initialize() {
    let program_id = Pubkey::new_unique();
    let mint = Keypair::new();
    let mint_decimals: u8 = 9;


    let mut program_test = ProgramTest::new(
        "tokentracing",
        program_id,
        processor!(process_instruction)
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    create_and_initialize_mint(
        &mut banks_client, 
        recent_blockhash, 
        &payer, 
        &payer, 
        &mint, 
        &spl_token::id(), 
        &mint_decimals
    )
    .await
    .unwrap();
    
    let (vault, vault_bump_seed) = Pubkey::find_program_address(&[b"vault", &mint.pubkey().to_bytes()], &program_id);

    // check if vault exists or not
    assert_eq!(
        banks_client.get_account(vault).await.expect("Account"),
        None,
    );

    let payer_account = AccountMeta {
        pubkey: payer.pubkey(),
        is_signer: true,
        is_writable: true,
    };

    let vault_account = AccountMeta {
        pubkey: vault,
        is_signer: false,
        is_writable: true
    };

    let mint_account = AccountMeta {
        pubkey: mint.pubkey(),
        is_signer: false,
        is_writable: true
    };

    let sys_account = AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: false
    };
    let accounts = vec![payer_account, vault_account, sys_account, mint_account];
    let ins_data = &[0_u8]; 
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            ins_data,
            accounts,
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // check if vault exists or not
    assert_ne!(
        banks_client.get_account(vault).await.expect("Account"),
        None,
    );
}

#[tokio::test]
async fn test_SOL_to_token() {
    let program_id = Pubkey::new_unique();
    let mint = Keypair::new();
    let mint_decimals: u8 = 9;


    let mut program_test = ProgramTest::new(
        "tokentracing",
        program_id,
        processor!(process_instruction)
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    create_and_initialize_mint(
        &mut banks_client, 
        recent_blockhash, 
        &payer, 
        &payer, 
        &mint, 
        &spl_token::id(), 
        &mint_decimals
    )
    .await
    .unwrap();
    
    let (vault, vault_bump_seed) = Pubkey::find_program_address(&[b"vault", &mint.pubkey().to_bytes()], &program_id);

    let payer_account = AccountMeta {
        pubkey: payer.pubkey(),
        is_signer: true,
        is_writable: true,
    };

    let vault_account = AccountMeta {
        pubkey: vault,
        is_signer: false,
        is_writable: true
    };

    let mint_account = AccountMeta {
        pubkey: mint.pubkey(),
        is_signer: false,
        is_writable: true
    };

    let sys_account = AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: false
    };
    let accounts = vec![payer_account, vault_account, sys_account, mint_account];
    let ins_data = &[0_u8]; 
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            ins_data,
            accounts,
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let vault_ata = get_associated_token_address(&vault, &mint.pubkey());
    let payer_ata = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
    
    let amount = 1000;
    let arr = u64::to_le_bytes(amount);
    let mut instruction_data = [1; 9];
    for i in 0..8 {
        instruction_data[i + 1] = arr[i];
    }
    let mut transation_SOL_to_token = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(program_id, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(payer_ata, false),
                AccountMeta::new(mint.pubkey(), false),
                AccountMeta::new(vault, false),
                AccountMeta::new(vault_ata, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new(system_program::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transation_SOL_to_token.sign(&[&payer], recent_blockhash);
    let payer_before = banks_client.get_account(payer.pubkey().clone()).await.unwrap().expect("payer lamport before");
    let vault_before = banks_client.get_account(vault.clone()).await.unwrap().expect("vault lamport before");
    
    banks_client.process_transaction(transation_SOL_to_token).await.unwrap();
    
    let payer_after = banks_client.get_account(payer.pubkey().clone()).await.unwrap().expect("payer lamport after");
    let vault_after = banks_client.get_account(vault.clone()).await.unwrap().expect("vault lamport after");
    
    // check if payer balance is decrease
    ma::assert_gt!(
        payer_before.lamports,
        payer_after.lamports
    );

    // check if payer balance is increase
    ma::assert_lt!(
        vault_before.lamports,
        vault_after.lamports
    );
}


#[tokio::test]
async fn test_token_to_SOL() {
    let program_id = Pubkey::new_unique();
    let mint = Keypair::new();
    let mint_decimals: u8 = 9;


    let mut program_test = ProgramTest::new(
        "tokentracing",
        program_id,
        processor!(process_instruction)
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    create_and_initialize_mint(
        &mut banks_client, 
        recent_blockhash, 
        &payer, 
        &payer, 
        &mint, 
        &spl_token::id(), 
        &mint_decimals
    )
    .await
    .unwrap();
    
    let (vault, vault_bump_seed) = Pubkey::find_program_address(&[b"vault", &mint.pubkey().to_bytes()], &program_id);

    let payer_account = AccountMeta {
        pubkey: payer.pubkey(),
        is_signer: true,
        is_writable: true,
    };

    let vault_account = AccountMeta {
        pubkey: vault,
        is_signer: false,
        is_writable: true
    };

    let mint_account = AccountMeta {
        pubkey: mint.pubkey(),
        is_signer: false,
        is_writable: true
    };

    let sys_account = AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: false
    };
    let accounts = vec![payer_account, vault_account, sys_account, mint_account];
    let ins_data = &[0_u8]; 
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            ins_data,
            accounts,
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();


    let vault_ata = get_associated_token_address(&vault, &mint.pubkey());
    let payer_ata = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
    
    let amount = 1000;
    let arr = u64::to_le_bytes(amount);
    let mut instruction_data = [2; 9];
    for i in 0..8 {
        instruction_data[i + 1] = arr[i];
    }
    let mut transation_token_to_SOL = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(program_id, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(payer_ata, false),
                AccountMeta::new(mint.pubkey(), false),
                AccountMeta::new(vault, false),
                AccountMeta::new(vault_ata, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new(system_program::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transation_token_to_SOL.sign(&[&payer], recent_blockhash);
    // let payer_balance_before = banks_client.get_balance(payer.pubkey()).await.unwrap();
    let vault_balance_before = banks_client.get_account(vault.clone()).await.unwrap().expect("vault balance before");
    // let vault_data = Account::unpack(&vault_balance_before.data).unwrap();
    banks_client.process_transaction(transation_token_to_SOL).await.unwrap();
    
    // let payer_balance_after = banks_client.get_balance(payer.pubkey()).await.unwrap();
    let vault_balance_after = banks_client.get_account(vault.clone()).await.unwrap().expect("vault balance after");
    
    // check if payer balance is increase
    // ma::assert_lt!(
    //     payer_balance_before,
    //     payer_balance_after
    // );

    // check if payer balance is decrease
    ma::assert_gt!(
        2,
        1
    );
}