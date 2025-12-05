use anchor_lang::prelude::*;

use crate::state::EscrowState;

pub mod errors;
pub mod state;

declare_id!("8UNhAJrkKqKk6gYxTzHmbSK5BHy3SJFzgQz3Ctt8LKkR");

#[program]
pub mod day10_escrow_anchor {
    use super::*;

    // Initializing Escrow Vault.
    pub fn initialze_escrow(ctx: Context<InitializeEscrow>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    // Defining the Signer who will initialize the Escrow account
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        seeds = [b"vault", initializer.key().as_ref()],   // Used to find the PDA for the ATA internally using -> Pubkey::find_program_address(seeds), the PDA will be used to sign transactions by the Vault or ATA
        bump, // This is the bump which is found while finding the PDA (as extra value), will be needed when using PDA to sign transactions
    )]
    pub vault_authority: AccountInfo<'info>,

    // Creating account that can act as Vault to lock in the transaction. creating an Associated Token Account(ATA)
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    // Storing Metadata for the Escrow
    #[account(
        init,
        payer = initializer,
        space = 8 + std::mem::size_of::<EscrowState>(), // Anchor account discriminator(Unique prefix for Anchor Accounts) + Rust compile-time size of the struct
    )]
    pub escrow_state: Account<'info, EscrowState>,

    // Mint of the token that will go into Escrow
    pub mint: Account<'info, Mint>,

    // Rent and System programs
    // System program is used to init accounts
    pub system_program: Program<'info, System>,
    // Token program is used to init associated token accounts and managing the transfers
    pub token_program: Program<'info, Token>,
    // Rent exemption calculation for the accounts created
    pub rent: Sysvar<'info, Rent>,
}
