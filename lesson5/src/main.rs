// Ownership, Borrowing and References

// Ownership
-----------------------------------------
// Why Ownership Exists
// Rust is a sytems programming language and like C++ and C, gives you access to the system's memoty, but C++ and C are not memory efficient. Thats where Rust wins.

// They allow you to use a part of the memory, after which you should free them. The problem is that it creates bugs, you might free the memory more than once, or you would forget to free the memory. This issue was solved using garbage collector. Which is done at the runtime, in the procees stops the program. Freezes the program for a little while. This causes a slow performance and inefficient outcome.

// The solution, OWNERSHIP - RUST

--------------------------------------------------
// Ownership Rules
--------------------------------------------------
// 1. Each value in Rust has an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped out.

// 1. Each value in Rust has an owner.
fn main() {
    let s1 = String::from("RUST");
    ley len = calculate_length(&s1);
};

fn calculate_length(s: &String) -> usize {
    s.len();
}
