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
    // Slices: [1, 2, 3, 4, 5];
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices)
}
