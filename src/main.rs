#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process, str::SplitWhitespace};

fn main() {
    // Get PATH environment variable or use default
    let path = env::var("PATH").unwrap_or("/usr/bin:/usr/local/bin".to_string());
    // Split PATH into a list of paths
    let path_list = path.split(':').collect::<Vec<&str>>();

    let stdin = io::stdin();
    let mut input = String::new();

    // Wait for user input
    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        let mut splitted_input = input.trim().split_whitespace();

        match splitted_input.next() {
            Some(input_command) => match input_command {
                "exit" => exit_handler(splitted_input),
                "echo" => {
                    let text = splitted_input.collect::<Vec<&str>>().join(" ");
                    println!("{}", text);
                }
                "type" => type_handler(splitted_input, &path_list),
                "pwd" => pwd_handler(),
                "cd" => cd_handler(splitted_input),
                input_command => {
                    let args = splitted_input.collect::<Vec<&str>>();
                    custom_command_handler(input_command, &args, &path_list)
                }
            },
            _ => {
                eprintln!("{}: command not found", input.trim());
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

fn type_handler(mut splitted_input: SplitWhitespace, path_list: &[&str]) {
    let command = splitted_input.next().unwrap();

    match command {
        "exit" | "echo" | "type" | "pwd" | "cd" => {
            println!("{} is a shell builtin", command);
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

            eprintln!("{}: not found", command);
        }
    }
}

fn pwd_handler() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("{}", current_dir.display());
}

fn cd_handler(mut splitted_input: SplitWhitespace) {
    let new_dir = splitted_input.next().unwrap();

    let path = env::current_dir().expect("Failed to get current directory");
    let new_path = path.join(new_dir);

    if new_path.is_dir() {
        env::set_current_dir(new_path).expect("Failed to change directory");
    } else {
        eprintln!("cd: {}: No such file or directory", new_dir);
    }
}

fn custom_command_handler(command: &str, args: &[&str], path_list: &[&str]) {
    // Check if the command exists in the path
    let command_path = path_list.iter().find(|path| {
        let full_path = format!("{}/{}", path, command);
        std::fs::metadata(&full_path).is_ok()
    });

    // Execute the command if it exists
    match command_path {
        Some(path) => {
            execute_command(&format!("{}/{}", path, command), args);
        }
        None => {
            eprintln!("{}: command not found", command);
        }
    }
}

fn execute_command(command: &str, args: &[&str]) {
    let mut child = process::Command::new(command)
        .args(args)
        .spawn()
        .expect("Failed to execute command");

    child.wait().expect("Command failed to execute");
}
