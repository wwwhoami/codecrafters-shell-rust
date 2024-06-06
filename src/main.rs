#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Wait for user input
    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        println!("{}: command not found", input.trim());
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
