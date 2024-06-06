#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Wait for user input
    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        let mut splitted_input = input.trim().split_whitespace();

        match splitted_input.next() {
            Some("exit") => {
                let code = splitted_input.next();

                match code {
                    Some(code) => {
                        let code = code.parse::<i32>().unwrap();
                        process::exit(code);
                    }
                    None => {
                        process::exit(0);
                    }
                }
            }
            _ => {
                println!("{}: command not found", input.trim());
            }
        }

        // Clear input buffer and print prompt
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
