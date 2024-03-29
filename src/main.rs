use std::io;
use eval::{eval, Value};

fn main() {
    let mut user_input: String = String::new();

    println!("Enter a valid math operation.");
    println!("(for example 12 * 3 + 5)");
    println!("...");

    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    let user_input = user_input.trim();
    let result = eval(&user_input);

    match result {
        Ok(Value::Number(num)) => println!("Here's your result: {}", num),
        Err(error) => eprintln!("Error: {}", error),
        _ => eprintln!("Error or unsupported data type.")
    };
}