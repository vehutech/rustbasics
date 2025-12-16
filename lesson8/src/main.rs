// Demonstration on one mutable reference or many immutable references.

// STRUCT:
// A struct is a data structure that allows you to group multiple fields together under one name.

// Define a struct to represent a bank account
// Structure representing a simple bank account
// Contains basic account information for a single user
struct AccountModel {
    bank_name: String,  // Name of the bank holding the account
    ac_name: String,    // Account holder's full name
    ac_num: u64,        // Account number (stored as integer - note: loses leading zeros)
    ac_bal: f64,        // Account balance in currency units (using float for decimal amounts)
}

impl AccountModel {
    // Method to change the account holder's name
    // Uses generic trait bound <S: Into<String>> to accept both &str and String
    // Takes mutable reference (&mut self) since it modifies the account name
    fn change_ac_name<S: Into<String>>(&mut self, input_name: S){
        // Convert the input to String type (handles &str or String automatically)
        let new_name = input_name.into();
        
        // Display confirmation message showing old and new names
        println!("You are about to change your name from {} to {}", self.ac_name, new_name);
        
        // Update the account name field with the new name
        self.ac_name = new_name;
    }

    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {}", amount, self.ac_bal);
        self.ac_bal -= amount
    }
    
    // Method to display all account details to the console
    // Takes immutable reference (&self) since it only reads data, doesn't modify
    fn display_ac_details(&self){
        println!("WELCOME TO: {}", self.bank_name);
        println!("ACCOUNT NAME: {}", self.ac_name);
        println!("ACCOUNT NUMBER: {}", self.ac_num);
        println!("ACCOUNT BALANCE: {}", self.ac_bal);
    }
}

fn main(){
    // Create a new mutable account instance
    // 'mut' keyword allows us to modify the account later (for name change)
    let mut new_acc = AccountModel {
        bank_name: "ZENITH BANK".to_string(),           // Initialize bank name
        ac_name: "Adam Coker".to_string(),     // Initialize account holder name
        ac_num: 1234567678,                              // Initialize account number
        ac_bal: 600_000.00                               // Initialize balance (underscore for readability)
    };
    
    // Display the initial account details
    new_acc.display_ac_details();
    
    // Change the account holder's name (demonstrates &mut self method)
    new_acc.change_ac_name("Jon Gjengest");
    
    // Display updated account details to confirm name change
    new_acc.display_ac_details();

    // Withdraw money
    new_acc.withdraw(500_000.0);
}