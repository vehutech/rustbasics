// Demonstration on one mutable reference or many immutable references.

// STRUCT:
// A struct is a data structure that allows you to group multiple fields together under one name.

struct BankAccount {
    bank_name: String,
    account_name: String,
    account_number: u64,
    balance: f64,
}
impl BankAccount {
    fn change_name<S: Into<String>>(&mut self, name: S) {
        println!("You are about to change your accoount name from {}, to: {:?}", self.account_name, name);
        self.account_name = name.into();
        println!("Name changed successfully to {}", name);
    }
    fn display_details(&self){
        println!("WELCOME TO {}", self.bank_name);
        println!("ACCOUNT NAME: {}", self.account_name);
        println!("ACCOUNT NUMBER: {}", self.account_number);
        println!("ACCOUNT BALANCE: {}", self.balance);
    }
}

fn main () {
    let mut account = BankAccount {
        bank_name: "ZENITH BANK".to_string(),
        account_name: "Daniel Adavi Alonge".to_string(),
        account_number: 1432786533,
        balance: 600000.5
    };
    account.change_name("Daniel Vehu Alonge");
    account.display_details();
}