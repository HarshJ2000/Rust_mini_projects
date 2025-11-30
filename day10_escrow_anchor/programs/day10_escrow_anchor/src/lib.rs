use anchor_lang::prelude::*;

mod errors;

declare_id!("8UNhAJrkKqKk6gYxTzHmbSK5BHy3SJFzgQz3Ctt8LKkR");

#[program]
pub mod day10_escrow_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
