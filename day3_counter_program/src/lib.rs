use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// AccountInfo -> represents an account
// next_account_info -> used to iterate over accounts
// entrypoint -> a macro which is used to register program's entry function
// ProgramResult -> returns Result<(), ProgramError> and can be used to return result from entrypoint function
// msg! -> macro used to log a message to program logs
// Pubkey -> Publickey

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

entrypoint!(counter_contract); // Registers counter_contract function as entrypoint, Solana runtime will call this exported function when a transaction hits the program id
pub fn counter_contract(
    _program_id: &Pubkey,     // publickey of the deployed program
    accounts: &[AccountInfo], // getting all accounts as a reference of array of accountInfo's
    instruction_data: &[u8],  // raw bytes included by client in the transaction
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?; // iterating to get to first account
    let instruction_type = InstructionType::try_from_slice(instruction_data)?; // deserializing instruction_data using borsh for InstructionType enum (try_from_slice is helping with deserialization)
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?; // Deserializing &acc.data.borrow()

    // ? -> used for error handling, returns error when a

    match instruction_type {
        // Pattern-matching for instruction types
        InstructionType::Decrement(value) => {
            msg!("Decrementing......");
            counter_data.count -= value;
        }
        InstructionType::Increment(value) => {
            msg!("Incrementing......");
            counter_data.count += value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?; // Serializing the deserialized 
    msg!("Counter updated to: {}", counter_data.count);

    Ok(())
}
