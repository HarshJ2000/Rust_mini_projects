pub struct Account {
    account_number: u32,
    balance: f64,
}

// pub enum TransactionType {
//     DepositMoney(u64),
//     WithdrawMoney(u64),
// }

pub enum TransactionErr {
    InsufficientBalance,
    InvalidAmt,
}

impl Account {
    pub fn new(account_number: u32, balance: f64) -> Self {
        Account {
            account_number,
            balance: 0.0,
        }
    }

    pub fn depositMoney(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited: {}, New balance: {}", amount, self.balance);
        } else {
            println!("Invalid Amount!!!!!");
        }
    }
}

fn main() {
    let user1 = Account {
        account_number: 00000000,
        balance: 1000.00,
    };

    println!("{}", user1.account_number);
    println!("{}", user1.balance);
    println!("{}", user1.balance);
}
