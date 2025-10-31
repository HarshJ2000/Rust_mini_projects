use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)] // For serializing and deserializing 
struct Counter {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    // Options created using enum for instructions
    Increment(u32),
    Decrement(u32),
}

entrypoint!(counter_contract);
pub fn counter_contract(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Decrement(value) => {
            msg!("Decrementing......");
            counter_data.count -= value;
        }
        InstructionType::Increment(value) => {
            msg!("Incrementing......");
            counter_data.count += value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Counter updated to: {}", counter_data.count);

    Ok(())
}
