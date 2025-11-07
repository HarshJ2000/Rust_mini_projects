use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter {
    owner: Pubkey,
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    Initialize,
    Increment(u32),
    Decrement(u32),
    Reset,
}

entrypoint!(counter_program);
pub fn counter_program(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut accounts_iter = accounts.iter();

    let counter_account = next_account_info(&mut accounts_iter)?;
    let signer = next_account_info(&mut accounts_iter)?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    let mut counter_data = if counter_account.data_len() == 0 {
        Counter {
            owner: Pubkey::default(),
            count: 0,
        }
    } else {
        Counter::try_from_slice(&counter_account.data.borrow())?
    };

    match instruction_type {
        InstructionType::Initialize => {
            msg!("Initializing counter account.....");
            counter_data.owner = *signer.key;
            counter_data.count = 0;
            msg!("Counter initialized with owner: {}", counter_data.owner);
        }
        InstructionType::Increment(value) => {
            msg!("Executing Increment....");
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            msg!("Executing Decrement....");
            counter_data.count -= value;
        }
        InstructionType::Reset => {
            msg!("Attempting to reset counter....");
            if counter_data.owner != *signer.key {
                msg!("Unauthorized reset attempt by: {}", signer.key);
                return Err(solana_program::program_error::ProgramError::IllegalOwner);
            }
            counter_data.count = 0;
            msg!("Counter reset by owner: {}", counter_data.owner);
        }
    }

    counter_data.serialize(&mut *counter_account.data.borrow_mut())?;
    println!("Counter updated: Count = {}", counter_data.count);

    Ok(())
}
