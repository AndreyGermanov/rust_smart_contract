trait Account {
    fn deposit(&mut self, amount: i32);
    fn withdraw(&mut self, amount: i32);
    fn balance(&self) -> i32;
}

struct BankAccount{
    account_number: u32,
    holder_name: String,
    balance: i32    
}

impl BankAccount {
    fn new(account_number: u32, holder_name: String,balance:i32) -> Self {
        BankAccount{account_number, holder_name,balance}
    }
}

impl Account for BankAccount {
        
    fn deposit(&mut self, amount: i32) {
        self.balance += amount
    }
    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount
    }
    fn balance(&self) -> i32 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount::new(123456, "Andrei Germanov".to_string(),100);
    let mut account2 = BankAccount::new(654321, "John Doe".to_string(),100);
    account1.deposit(100);
    account2.withdraw(100);
    println!("{},{}",account1.balance(),account2.balance())
}
