use crate::contexts::DepositTokens;
use crate::errors::*;
use crate::state::assert_initializer;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<DepositTokens>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_state;

    msg!("--- Deposit Attempt ---");
    msg!("caller: {}", ctx.accounts.initializer.key());
    msg!("escrow_state_before: {:?}", escrow.state);

    assert_initializer(&ctx.accounts.initializer, &escrow.initializer)?;

    // Validating if the escrow state is Initialized or not? If not then return Error-> InvalidState
    require!(escrow.can_deposit(), EscrowError::InvalidState);

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
    msg!("deposit_amount: {}", escrow.initializer_amount);
    msg!("escrow_state_after: Deposited");

    Ok(())
}
