use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::deposit::DepositTokens;
use crate::instructions::initialize::InitializeEscrow;
use crate::instructions::withdraw::WithdrawTokens;

declare_id!("GWHDeVEboCXJfKZyuZuKX3omooMYYjMXs9SdRYa7HVNB");

pub mod day10_escrow_anchor {

    use super::*;

    // Initializing Escrow Vault.
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        initializer_amount: u64,
        taker_amount: u64,
        expiry: i64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, initializer_amount, taker_amount, expiry)
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>) -> Result<()> {
        instructions::deposit::handler(ctx)
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>) -> Result<()> {
        instructions::withdraw::handler(ctx)
    }
}
