// Functions - The function main is the entry point. if you change it, you will encounter critical error.

// Every function begins with the reserved word fn which stands for function, the name of the function, followed by paranthesis then a code block.

// Function or variable names must be written in snake case - hello_world or kebab case -  hello-world

// Rust supports hoisting - a programming style in which function can be called from anywhere, even before its definition. Hoisting exists in javascript too

fn main(){
    hello_woorld();
    tell_height(50);
    human_id("Vehu", 30, 170.0);

    let price = 5;
    let qty = 10;
    let _x  = {
        price * qty
    };

    println!("Result of {} * {} is: {}", price, qty, _x);

    let y = add(4, 6);
    println!("The value of y is {}", y);
    println!("Value from function 'add' is: {}", add(4, 6))
}

fn hello_woorld(){
    println!("Helo Rust!");
}

// you can insert input values

fn tell_height(height: i32) {
    println!("My height is: {}cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {},\nI am {}, years old\nMy height is {}cm", name, age, height)
}

// Expressions and Statements

// Expressions are anything that return a value.
// Statements are anything that does not return a value


// Functions retunrs values too
fn add(a: i32, b:i32) -> i32 {
    a+b
}