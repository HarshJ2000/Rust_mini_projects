use anchor_lang::prelude::*;

// Storing states required for Escrow inside an account
#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub expiry: i64,
    pub bump: u8,
    pub state: EscrowStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Clone, Eq)]
pub enum EscrowStatus {
    Initialized,
    Deposited,
    Completed,
    Cancelled,
}
