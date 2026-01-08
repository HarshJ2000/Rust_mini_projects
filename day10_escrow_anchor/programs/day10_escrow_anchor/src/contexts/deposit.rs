use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::EscrowState;

// Constraints for deposit_tokens instruction
#[derive(Accounts)]
pub struct DepositTokens<'info> {
    // Initializer Account
    #[account(mut)]
    pub initializer: Signer<'info>,

    // Escrow state used for checking ->  if Initializer is valid?
    #[account(
        mut,
        constraint = escrow_state.initializer == initializer.key(),
    )]
    pub escrow_state: Account<'info, EscrowState>,

    /// CHECK: This is a PDA derived from known seeds.
    /// It is used only as a signing authority and holds no data.
    #[account(
        seeds = [b"vault", initializer.key().as_ref()],
        bump = escrow_state.bump,
    )]
    pub vault_authority: AccountInfo<'info>,

    // Vault ATA used to store the tokens
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    // Initializer ATA where the initializer's tokens are stored and from here will be transferred to the Vault ATA
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = initializer,
    )]
    pub initializer_ata: Account<'info, TokenAccount>,

    // Mint of the tokens in the Vault ATA and Initializer ATA
    pub mint: Account<'info, Mint>,

    // Token Program used to create and manage ATA's
    pub token_program: Program<'info, Token>,
}
