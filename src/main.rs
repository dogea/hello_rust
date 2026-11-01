use std::io;

fn main() {
    println!("'what is your name?");

    // Create a new mutable String to store input
    let mut name = String::new();

    // Read input from the terminal
    // This prompts users to enter their name in the terminal
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    // Trim whitespace (like newline) from the input
    let name = name.trim();

    // Print the personalized messagw
    println!("Welcome {} - this is your first Rust program!", name);
}
