use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn increment(&mut self) {
        self.count += 2;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}
