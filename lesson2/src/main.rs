// The 4 Primitive Data Types in Rust.

//  int, float, bool, char
// Integer

// Rust has signed (+ or -) and unsigned (only+) integer data type.

// signed integer "i" (+ or -) types of different sizes.
// i8, i16, i32, i64, i128: -Signed integers.

// unsigned integer "u"
// u8, u16, u32, u64, u128: -Unsigned integers.
fn main(){
    let x: i32 = -45;
    let y: u64 = 200;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    // Difference between i32 (32 bits) and i64 (64 bits)
    // Range: 
    // i32 - 2(pow(31)) = 2147483647
    // i64 - 2(pow(63)) = 9223372036854775807

    let e: i32 = 2147483647;
    let i: i32 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    let k: i32 = 2147483648;
    let t: i32 = 9223372036854775809;

    println!("Maximum value of i32: {}", k);
    println!("Maximum value of i64: {}", t);

    // Floats [Floating point Types]
    // numbers with fractional parts, only two types of float in rust - f32, f64

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // Boolean Values: true, false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // Character Type - char
    let letter: char = 'a';
    println!("The first letter of the alphabet: {}", letter);


}
