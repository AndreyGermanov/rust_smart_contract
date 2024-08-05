trait Account {
    fn deposit(&mut self, amount: i32) -> Result<(),String>;
    fn withdraw(&mut self, amount: i32) -> Result<(), String>;
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
        
    fn deposit(&mut self, amount: i32) -> Result<(),String> {
        self.balance += amount;
        Ok(())
    }
    fn withdraw(&mut self, amount: i32) -> Result<(),String> {
        if amount > self.balance {
            return Err("There are no enough balance".to_string())
        }
        self.balance -= amount;
        Ok(())
    }
    fn balance(&self) -> i32 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount::new(123456, "Andrei Germanov".to_string(),100);
    let mut account2 = BankAccount::new(654321, "John Doe".to_string(),0);
    match account1.deposit(100) {
        Ok(()) => println!("Done!"),
        Err(message) => println!("Error: {}",message)
    }
    match account2.withdraw(100) {
        Ok(()) => println!("Done!"),
        Err(message) => println!("Error: {}",message)
    }
    println!("{},{}",account1.balance(),account2.balance())
}
