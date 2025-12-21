use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::EscrowState;

pub mod errors;
pub mod state;

declare_id!("GWHDeVEboCXJfKZyuZuKX3omooMYYjMXs9SdRYa7HVNB");

#[program]
pub mod day10_escrow_anchor {
    use super::*;
    use crate::{errors::EscrowError, state::EscrowStatus};

    // Initializing Escrow Vault.
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        initializer_amount: u64,
        taker_amount: u64,
        expiry: i64,
    ) -> Result<()> {
        // Validating valid amount
        if initializer_amount == 0 || taker_amount == 0 {
            return err!(EscrowError::InvalidAmount);
        }

        // Validating escrow not expired
        let clock = Clock::get()?;
        if expiry <= clock.unix_timestamp {
            return err!(EscrowError::ExpiredEscrow);
        }

        // Setting Escrow States, which will be needed later while interacting with the escrow vault
        let escrow_state = &mut ctx.accounts.escrow_state;
        escrow_state.initializer = ctx.accounts.initializer.key();
        escrow_state.initializer_amount = initializer_amount;
        escrow_state.taker_amount = taker_amount;
        escrow_state.expiry = expiry;
        escrow_state.bump = ctx.bumps.vault_authority;
        escrow_state.state = EscrowStatus::Initialized;

        Ok(())
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_state;

        // Validating if Initializer is authorized or not?
        require!(
            escrow.state == EscrowStatus::Initialized,
            EscrowError::Unauthorized
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

        Ok(())
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_state;

        require!(
            escrow.state == EscrowStatus::Deposited,
            EscrowError::Unauthorized
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
        escrow.state = EscrowStatus::Cancelled;

        Ok(())
    }
}

// Constraints for initialize_escrow Function or instruction
#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    // Defining the Signer who will initialize the Escrow account
    #[account(mut)]
    pub initializer: Signer<'info>,

    /// CHECK: This is a PDA derived from known seeds.
    /// It is used only as a signing authority and holds no data.
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
    // Associated token account is needed because we've created vault ATA
    pub associated_token_program: Program<'info, AssociatedToken>,
    // Rent exemption calculation for the accounts created
    pub rent: Sysvar<'info, Rent>,
}

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
