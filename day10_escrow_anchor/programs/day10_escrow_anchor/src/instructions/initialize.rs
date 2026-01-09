use crate::contexts::InitializeEscrow;
use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn handler(
    ctx: Context<InitializeEscrow>,
    initializer_amount: u64,
    taker_amount: u64,
    expiry: i64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_state;

    msg!("--- Escrow Initialization ---");
    msg!("initializer: {}", ctx.accounts.initializer.key());
    msg!("initializer_amount: {}", initializer_amount);
    msg!("taker_amount: {}", taker_amount);
    msg!("expiry: {}", expiry);

    require!(
        escrow.state != EscrowStatus::Initialized,
        EscrowError::AlreadyInitialized
    );

    // Validating valid amount
    if initializer_amount == 0 || taker_amount == 0 {
        return err!(EscrowError::InvalidAmount);
    }

    // Validating escrow not expired
    let clock = Clock::get()?;
    require!(
        expiry > clock.unix_timestamp + 60,
        EscrowError::ExpiredEscrow
    );

    // Setting Escrow States, which will be needed later while interacting with the escrow vault
    let escrow_state: &mut Account<'_, EscrowState> = &mut ctx.accounts.escrow_state;
    escrow_state.initializer = ctx.accounts.initializer.key();
    escrow_state.initializer_amount = initializer_amount;
    escrow_state.taker_amount = taker_amount;
    escrow_state.expiry = expiry;
    escrow_state.bump = ctx.bumps.vault_authority;
    escrow_state.state = EscrowStatus::Initialized;

    msg!("Escrow Initialized...");
    msg!("Initializer: {}", escrow_state.initializer);
    msg!("escrow_state: Initialized");

    Ok(())
}
