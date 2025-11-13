use serialization_lib::Counter;
use solana_program::msg;

pub fn process_instruction() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    msg!("Counter after incremented: {}", counter.count);
}
