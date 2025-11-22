use anchor_lang::prelude::*;

declare_id!("AzrUyx3XLhFWipfNMfPQgjjQMZcsiLcJ6KY2VA5Mpbnj");

// Program Logic
#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial: u64) -> Result<()> {
        let ctr = &mut ctx.accounts.counter_acc;
        ctr.count = initial;
        ctr.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn increment(ctx: Context<Mutate>, value: u64) -> Result<()> {
        let ctr = &mut ctx.accounts.counter_acc;
        ctr.count = ctr.count.checked_add(value).ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    pub fn decrement(ctx: Context<Mutate>, value: u64) -> Result<()> {
        let ctr = &mut ctx.accounts.counter_acc;
        if ctr.count == 0 {
            return Err(ErrorCode::CannotGoBelowZero.into());
        }
        ctr.count = ctr.count.checked_sub(value).ok_or(ErrorCode::Underflow)?;
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let ctr = &mut ctx.accounts.counter_acc;
        ctr.count = 0;
        Ok(())
    }
}

// Data account creation
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, payer = authority, space = 8+8+32,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter_acc: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Mutate into Data Accounts
#[derive(Accounts)]
pub struct Mutate<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter_acc: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

// Resetting Counter Account Struct
#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter_acc: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

// Account Data Account
#[account]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}

// Error handling
#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic Overflow")]
    Overflow,
    #[msg("Arithmetic Underflow")]
    Underflow,
    #[msg("Counter cannot go below zero!!!")]
    CannotGoBelowZero,
}
