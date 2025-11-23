use anchor_lang::prelude::*;

declare_id!("2B6GdZQ7aVzM93FrfpP5YMsqVaNehv7QuaupaX4QjkRp");

#[program]
pub mod anchor_pda_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("PDA Account: {}", ctx.accounts.user_pda.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"user", user.key().as_ref()],
        bump,
    )]
    pub user_pda: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub data: u64,
}
