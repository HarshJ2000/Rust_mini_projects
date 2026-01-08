use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

// Context for withdrawing tokens from vault
#[derive(Accounts)]
// WithdrawTokens context struct will have the same structure as the DepositTokens context struct because while sending and withdrawing we'll be using the same set of accounts
pub struct WithdrawTokens<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        mut,
        constraint = escrow_state.initializer == initializer.key()
    )]
    pub escrow_state: Account<'info, EscrowState>,

    /// CHECK: This is a PDA derived from known seeds.
    /// It is used only as a signing authority and holds no data.
    #[account(
        seeds = [b"vault", initializer.key().as_ref()],
        bump = escrow_state.bump,
    )]
    pub vault_authority: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = initializer,
    )]
    pub initializer_ata: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawTokens>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_state;

    msg!("Withdraw Started...");
    msg!("State before: {:?}", escrow.state);

    require!(
        escrow.state == EscrowStatus::Deposited,
        EscrowError::InvalidState
    );

    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= escrow.expiry,
        EscrowError::ExpiredEscrow
    );

    let amount = escrow.initializer_amount;

    // seperately defined initializer_key because we cannot call .key() inside the &[]
    let initializer_key = escrow.initializer.key();
    // Seeds reqruired for the signing of the withdraw transaction
    let seeds = &[b"vault", initializer_key.as_ref(), &[escrow.bump]];
    // signer_seeds is the way in which the transfer method of CPI wants the seeds to be provided
    let signer_seeds = &[&seeds[..]];

    // Building the cpi accounts required for transfer
    let cpi_accounts = anchor_spl::token::Transfer {
        from: ctx.accounts.vault_ata.to_account_info(),
        to: ctx.accounts.initializer_ata.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };

    // executing the cpi transfer
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        ),
        amount,
    )?;

    // updating the state for the escrow
    escrow.state = EscrowStatus::Refunded;

    msg!("Withdraw Successful...");
    msg!("State changed to: Cancelled");

    Ok(())
}
