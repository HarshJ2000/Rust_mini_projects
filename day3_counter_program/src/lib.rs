use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
};

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

pub fn counter_contract() -> ProgramResult {
    Ok(())
}
