// Variables and Mutability In Rust

// Variables are immutable by default (not constants);

// To be able to mutate or change the value of a variable it must be decalred as mut.
fn main () {
    println!("Hello, world!");
    let mut a:i32 = 5;
    println!("The value of a is {}", a);
    a = 10;
    println!("The new value of a is {}", a);
}