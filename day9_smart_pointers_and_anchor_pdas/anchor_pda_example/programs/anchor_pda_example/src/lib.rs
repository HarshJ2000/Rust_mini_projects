use anchor_lang::prelude::*;

declare_id!("2B6GdZQ7aVzM93FrfpP5YMsqVaNehv7QuaupaX4QjkRp");

#[program]
pub mod anchor_pda_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
