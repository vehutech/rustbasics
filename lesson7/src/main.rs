// Referencing and Borrowing
// Safety and performance
// Borrowing and references are powerful concepts in rust

// Understanding Refernces
// Borrowing and references are intertwined and means same thing.
// References allow you to borrow values without letting you take ownership
// References can be:
// 1. Immutable Reference
// 2. Mutable Reference
// You create a reference by adding "&"

fn main(){
    // Declare a mutable ariable as your luck number 
    let mut lucky_number = 42;
    // Reassign it as a mut ref
    let copy_of_my_lucky_number = &mut lucky_number;
    // mutate the value from the referenced variable
    *copy_of_my_lucky_number += 5;
    // print the value of the original variable
    println!("My lucky number is: {}", lucky_number);

}