use crate::contexts::WithdrawTokens;
use crate::errors::*;
use crate::state::assert_initializer;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<WithdrawTokens>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_state;

    msg!("--- Withdraw Attempt ---");
    msg!("caller: {}", ctx.accounts.initializer.key());
    msg!("escrow_state_before: {:?}", escrow.state);
    msg!("current_time: {}", Clock::get()?.unix_timestamp);
    msg!("expiry: {}", escrow.expiry);

    assert_initializer(&ctx.accounts.initializer, &escrow.initializer)?;

    require!(
        escrow.state == EscrowStatus::Deposited,
        EscrowError::InvalidState
    );

    let clock = Clock::get()?;
    require!(
        escrow.can_withdraw(clock.unix_timestamp),
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
    msg!("withdraw_amount: {}", escrow.initializer_amount);
    msg!("escrow_state_after: Refunded");

    Ok(())
}
