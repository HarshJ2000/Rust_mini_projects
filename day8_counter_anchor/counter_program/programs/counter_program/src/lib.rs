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


