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

        match input.trim().split_once(' ') {
            Some((command, args)) => match command {
                "exit" => match args {
                    "0" => break,
                    _ => println!("{}: command not found", input.trim()),
                },
                "echo" => println!("{}", args),
                "type" => match args {
                    "echo" | "exit" | "type" => println!("{} is a shell builtin", args),
                    _ => {
                        match find_executable_in_path(args) {
                            Some(path) => println!("{} is {}", args, path.to_string_lossy()),
                            None => println!("{}: not found", args),
                        }
                    },
                },
                _ => {
                    // Try to execute as an external command
                    match find_executable_in_path(command) {
                        Some(path) => {
                            // Split args by spaces to get individual arguments
                            let arg_parts: Vec<&str> = args.split_whitespace().collect();

                            // Execute the command with arguments
                            match Command::new(path).args(arg_parts).output() {
                                Ok(output) => {
                                    // Print the output
                                    io::stdout().write_all(&output.stdout).unwrap();
                                    io::stderr().write_all(&output.stderr).unwrap();
                                },
                                Err(_) => println!("Failed to execute: {}", command),
                            }
                        },
                        None => {
                            println!("{}: command not found", input.trim());
                        }
                    }
                }
            },
            None => {
                // No arguments
                let command = input.trim();
                if !command.is_empty() {
                    match command {
                        "exit" | "echo" | "type" => println!("{}: command not found", command),
                        _ => {
                            // Try to execute as an external command without arguments
                            match find_executable_in_path(command) {
                                Some(path) => {
                                    match Command::new(path).output() {
                                        Ok(output) => {
                                            // Print the output
                                            io::stdout().write_all(&output.stdout).unwrap();
                                            io::stderr().write_all(&output.stderr).unwrap();
                                        },
                                        Err(_) => println!("Failed to execute: {}", command),
                                    }
                                },
                                None => {
                                    println!("{}: command not found", command);
                                }
                            }
                        }
                    }
                } else {
                    println!("{}: command not found", input.trim());
                }
            }
        }
    }
}
