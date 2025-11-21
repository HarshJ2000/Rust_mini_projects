use anchor_lang::prelude::*;

declare_id!("AzrUyx3XLhFWipfNMfPQgjjQMZcsiLcJ6KY2VA5Mpbnj");

#[program]
pub mod counter_program{
    use super::*;

    pub fn initialize(ctx:Context<Initialize>) -> Result(<>){}

    pub fn increment(ctx:Context<Mutate>) -> Result(<>){}

    pub fn decrement(ctx:Context<Mutate>) -> Result(<>){}

    pub fn reset(ctx:Context<Reset>) -> Result(<>){}
}

// Data account creation
#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init, payer = signer, space = 8+8+32,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter_acc : Account<'info, Counter>,

    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program : Program<'info, System>,
}

// Mutate into Data Accounts
#[derive(Accounts)]
pub struct Mutate<'info>{
    #[account(
        mut,
        seeds = [b"counter", counter.authority.as_ref()],
        bump,
    )]
    pub counter_acc : Account<'info, Counter>,
    pub authority : Signer<'info>,
}