#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process, str::SplitWhitespace};

fn main() {
    // Get PATH environment variable or use default
    let path = env::var("PATH").unwrap_or("/usr/bin:/usr/local/bin".to_string());

    let stdin = io::stdin();
    let mut input = String::new();

    // Wait for user input
    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        let mut splitted_input = input.trim().split_whitespace();

        match splitted_input.next() {
            Some("exit") => exit_handler(splitted_input),
            Some("echo") => {
                let text = splitted_input.collect::<Vec<&str>>().join(" ");
                println!("{}", text);
            }
            Some("type") => type_handler(splitted_input, path.as_str()),
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

fn exit_handler(mut splitted_input: SplitWhitespace) {
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

fn type_handler(mut splitted_input: SplitWhitespace, path: &str) {
    let path_list = path.split(':').collect::<Vec<&str>>();

    let command = splitted_input.next().unwrap();

    match command {
        "exit" => {
            println!("exit is a shell builtin");
        }
        "echo" => {
            println!("echo is a shell builtin");
        }
        "type" => {
            println!("type is a shell builtin");
        }
        _ => {
            for path in path_list {
                // Check if the command exists in the path
                let full_path = format!("{}/{}", path, command);

                if std::fs::metadata(&full_path).is_ok() {
                    println!("{} is {}", command, full_path);
                    return;
                }
            }

            println!("{} not found", command);
        }
    }
}
