
use std::io;

fn main() {
    // Declare a mutable string variable to store user input
    let mut input_string = String::new();

    println!("Enter a string: ");

    // Read the user input from the standard input
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read input"); 

    // Trim whitespace (e.g., the trailing newline) from the input  
    let input_string = input_string.trim();  

    // Reverse the string using Rust's built-in string manipulation
    let reversed_string = input_string.chars().rev().collect::<String>();

    println!("Reversed string: {}", reversed_string);
}
