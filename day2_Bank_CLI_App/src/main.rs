pub struct Account {
    account_number: u32,
    balance: u64,
}

// pub enum TransactionType {
//     DepositMoney(u64),
//     WithdrawMoney(u64),
// }

#[derive(Debug)]
pub enum TransactionErr {
    InsufficientBalance,
    InvalidAmt,
}

impl Account {
    pub fn new(account_number: u32, balance: u64) -> Self {
        Account {
            account_number,
            balance: 0,
        }
    }

    pub fn deposit_money(&mut self, amount: u64) {
        if amount > 0 {
            self.balance += amount;
            println!("Deposited: {}, New balance: {}", amount, self.balance);
        } else {
            println!("Invalid Amount!!!!!");
        }
    }

    pub fn withdraw_money(&mut self, amount: u64) -> Result<(), TransactionErr> {
        // Used TransactionErr enum inside impl fn as a parameter
        if amount < 0 {
            return Err(TransactionErr::InvalidAmt);
        }

        let min_balance = 10;
        if self.balance - amount > min_balance {
            self.balance -= amount;
            println!("Withdrew: {}, New balance: {}", amount, self.balance);
            Ok(())
        } else {
            Err(TransactionErr::InsufficientBalance)
        }
    }
}

fn main() {
    let mut account1 = Account {
        account_number: 101,
        balance: 1000,
    };

    // Trying to deposit valid amount
    account1.deposit_money(1000);

    // Trying to deposit invalid amount
    account1.deposit_money(-1000);

    // attempting to withdraw money while maintaining minimum balance
    match account1.withdraw_money(500) {
        Ok(()) => println!("Withdraw Successful...."),
        Err(err) => println!("Withdraw failed: {:?}", err),
    }

    // attempting to withdraw money beyond balance
    match account1.withdraw_money(2500) {
        Ok(()) => println!("Withdraw Successful...."),
        Err(e) => println!("Withdraw Failed: {:?}", e),
    }
}
