use anchor_lang::prelude::*;

// Storing states for Escrow inside an account
#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub expiry: i64,
    pub bump: u8,
}
