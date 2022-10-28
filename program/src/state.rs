use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ExchangeAccount {
    pub admin: Pubkey,
    pub vault: Pubkey
}

pub const EXCHANGE_ACCOUNT_LEN: usize = size_of::<Pubkey>() * 2;
