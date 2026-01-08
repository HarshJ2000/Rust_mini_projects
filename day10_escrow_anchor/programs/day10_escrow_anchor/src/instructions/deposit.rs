use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

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

pub fn handler(ctx: Context<DepositTokens>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_state;

    msg!("Deposit Started...");
    msg!("State before: {:?}", escrow.state);

    // Validating if Initializer is authorized or not?
    require!(
        escrow.state == EscrowStatus::Initialized,
        EscrowError::InvalidState
    );

    // Validating Escrow Expiry
    let clock = Clock::get()?;
    require!(
        escrow.expiry > clock.unix_timestamp,
        EscrowError::ExpiredEscrow
    );

    // Getting amount used to initialize escrow
    let amount = escrow.initializer_amount;

    // Building CPI transfer accounts
    let cpi_accounts = anchor_spl::token::Transfer {
        from: ctx.accounts.initializer_ata.to_account_info(),
        to: ctx.accounts.vault_ata.to_account_info(),
        authority: ctx.accounts.initializer.to_account_info(),
    };

    // Reference to invoke the SPL Token Program
    let cpi_program = ctx.accounts.token_program.to_account_info();
    // Executing the CPI token transfer
    anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    // Updating the escrow state to deposited
    escrow.state = EscrowStatus::Deposited;

    msg!("Deposit successful...");
    msg!("State changed to: Deposited");

    Ok(())
}
