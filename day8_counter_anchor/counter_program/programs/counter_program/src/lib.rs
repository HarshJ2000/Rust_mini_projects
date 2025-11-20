use anchor_lang::prelude::*;

declare_id!("AzrUyx3XLhFWipfNMfPQgjjQMZcsiLcJ6KY2VA5Mpbnj");

// Program Logic
#[program]
pub mod counter_program{
    use super::*;

    pub fn initialize(ctx:Context<Initialize>) -> Result(<>){}

    pub fn increment(ctx:Context<Mutate>) -> Result(<>){}

    pub fn decrement(ctx:Context<Mutate>) -> Result(<>){}

    pub fn reset(ctx:Context<Reset>) -> Result(<>){}
}


// Data account creation
pub struct Initialize<'info>{
    #[account(
        init, payer = signer, space = 8+8+32,
        seeds = [b"counter", authority.key().as_ref()],
        bump
    )]
    pub new_account:Account<'info, NewAccount>

    #[account(mut)]
    pub signer: Signer<'info>
    pub program_id: Program<'info, System>
}
