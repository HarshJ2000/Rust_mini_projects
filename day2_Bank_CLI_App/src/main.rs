pub struct Account {
    account_number: u32,
    balance: f64,
}

pub enum TransactionType {
    DepositMoney(u64),
    WithdrawMoney(u64),
}

pub enum TransactionErr {
    InsufficientBalance,
    InvalidAmt,
}

fn main() {
    let user1 = Account {
        account_number: 00000000,
        balance: 1000.00,
    };

    println!("{}", user1.account_number);
    println!("{}", user1.balance);
}
