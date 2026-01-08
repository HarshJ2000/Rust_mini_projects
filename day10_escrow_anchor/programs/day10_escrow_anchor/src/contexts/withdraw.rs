use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::EscrowState;

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
