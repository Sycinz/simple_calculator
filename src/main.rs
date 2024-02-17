use std::io;
use eval::eval;

fn main() {
    let mut user_input = String::new();

    println!("Enter a valid math operation.");
    println!("(for example 12 * 3 + 5)");
    println!("...");

    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    let result = eval(&user_input);

    println!("Here's your result: {:?}", result);
}