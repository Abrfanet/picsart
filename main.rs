use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Enter a string to reverse: ");
    io::stdout().flush().unwrap();

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove the newline character at the end of the input
    input = input.trim().to_string();

    // Reverse the string
    let reversed = reverse_string(&input);

    // Output the reversed string
    println!("Reversed string: {}", reversed);
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
