// Ownership, Borrowing and References

// Ownership
// -----------------------------------------
// Why Ownership Exists
// Rust is a sytems programming language and like C++ and C, gives you access to the system's memoty, but C++ and C are not memory efficient. Thats where Rust wins.

// They allow you to use a part of the memory, after which you should free them. The problem is that it creates bugs, you might free the memory more than once, or you would forget to free the memory. This issue was solved using garbage collector. Which is done at the runtime, in the procees stops the program. Freezes the program for a little while. This causes a slow performance and inefficient outcome.

// The solution, OWNERSHIP - RUST

// --------------------------------------------------
// Ownership Rules
// --------------------------------------------------
// 1. Each value in Rust has an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped out.

// 1. Each value in Rust has an owner.
// fn main() {
//     // Get the string
//     let s1 = String::from("D Vehu Alonge");
//     let string_len = calculate_string_length(&s1);
//     println!("The length of the string - {}, is - {}.", s1, string_len)
// }

// // function to calculate the length of the string
// fn calculate_string_length(s: &String) -> usize {
//     s.len()
// }

// There can only be one owner at a time.
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1;

//     println!("{}", s2);
// }

// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    let s1 = String::from("RUST");
    let len = calculate_string_length(&s1);
    println!("Length of '{}' is '{}'.", s1, len);
    // print_lost();
}

// s1 goes out of scope and its value will be dropped in this scope.
// fn print_lost() {
//     println!("{}", &s1);
// }


fn calculate_string_length(s: &String) -> usize {
    s.len()
}