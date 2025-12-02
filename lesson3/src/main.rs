// Compound Data Types
// Arrays, Tuples, Slices and Strings (String slice)

fn main() {
    // 1. Arrays
    // Arrays are fixed size of collection of elements of homogenous types

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    // let mix = [1, 2, "apple", true];
    // The code commented out will not run cos its data element has mixed type. Arrays is rust language is of homogenous type.

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array - First element: {}", fruits[0]);
    println!("Fruits Array - Second element: {}", fruits[2]);
    println!("Fruits Array - Third element: {}", fruits[2]);

    // 2. Tuples
    // Tuples contain heterogenous collection of elements of fixed size.
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    // A mixed tuple
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My mix tuple: {:?}", my_mix_tuple);

    // 3. Slices
    // Slices are a dynamicly size view into contagious sequence of element
    // Slices: [1, 2, 3, 4, 5]; -- this is a contagious sequence
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book SLice: {:?}", book_slices);

    let fruit_list: &[&String] = &[&"Apple".to_string(), &"Banana".to_string(), &"Pawpaw".to_string()];
    println!("A list of fruits i like: {:?}", fruit_list);

    // Strings Vs String Slieces (&str)
    
    // Strings `: String` are mutable, you can change them, they are owned, not borrowed. Memory allocation and memory efficiency is very important in rust programming language.

    // When you declare a string variable it is allocated on the heep not stack
    // These string objects can grow or shrink, you can add or delete. memmory allocation is done at run time and dynamic in the heep. But it is slow. its not fixed.

    // Any data type in rust by default is immutable - you cannot change it.

    let mut stone_cold: String = String::from("Hello, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

     // String slices `: &str` are stored on the stack not the heap. and are faster. Its a reference and not an owned string. They are immutable. you cannot modify anything. it is good for memory efficiency.

     let greeting: String = String::from("Hello, World!");
     let greeting_slice: &str = &greeting[0..5]; 
    //  0..5 prints the first 5 characters of the string value
     println!("Greeting slice value: {}", greeting_slice);
}


/*Rust cleanses any variable that is used outside its defined scope
the block below will fail compile
fn print() {
    println!("OUT OF SCOPE VARIABLE: {}", greeting);
}*/