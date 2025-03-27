#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use pathsearch::find_executable_in_path;

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("$ ");
        stdout.flush().unwrap();
        stdin.read_line(&mut input).unwrap();

        let input_trimmed = input.trim();

        if input_trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input_trimmed.split_whitespace().collect();
        let command = parts[0];

        match command {
            "exit" => {
                if parts.len() > 1 && parts[1] == "0" {
                    break;
                } else {
                    println!("{}: command not found", input_trimmed);
                }
            },
            "echo" => {
                if parts.len() > 1 {
                    println!("{}", &input_trimmed[5..]);
                } else {
                    println!();
                }
            },
            "type" => {
                if parts.len() > 1 {
                    let arg = parts[1];
                    match arg {
                        "echo" | "exit" | "type" => println!("{} is a shell builtin", arg),
                        _ => {
                            match find_executable_in_path(arg) {
                                Some(path) => println!("{} is {}", arg, path.to_string_lossy()),
                                None => println!("{}: not found", arg),
                            }
                        },
                    }
                } else {
                    println!("{}: command not found", input_trimmed);
                }
            },
            _ => {
                // Try to execute as an external command
                match find_executable_in_path(command) {
                    Some(path) => {
                        // Get all arguments (excluding the command)
                        let args = if parts.len() > 1 {
                            &parts[1..]
                        } else {
                            &[]
                        };

                        // Create a command that properly uses the command name, not the path
                        match Command::new(&path).args(args).output() {
                            Ok(output) => {
                                // Print the output
                                io::stdout().write_all(&output.stdout).unwrap();
                                io::stderr().write_all(&output.stderr).unwrap();
                            },
                            Err(_) => println!("Failed to execute: {}", command),
                        }
                    },
                    None => {
                        println!("{}: command not found", input_trimmed),
                    }
                }
            }
        }
    }
}
