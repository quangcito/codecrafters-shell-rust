#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use pathsearch::find_executable_in_path;
use std::os::unix::fs;

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
                        // Build the command arguments
                        let mut cmd_args = String::new();
                        for part in &parts[1..] {
                            if !cmd_args.is_empty() {
                                cmd_args.push(' ');
                            }
                            // Escape any spaces or special characters
                            if part.contains(' ') || part.contains('"') || part.contains('\'') {
                                cmd_args.push_str(&format!("\"{}\"", part.replace("\"", "\\\"")));
                            } else {
                                cmd_args.push_str(part);
                            }
                        }

                        // Use a shell script with a symbolic link to preserve the command name
                        let sh_cmd = format!(
                            "TEMP_DIR=$(mktemp -d) && \
                             ln -sf {} $TEMP_DIR/{} && \
                             $TEMP_DIR/{} {} && \
                             rm -rf $TEMP_DIR",
                            path.to_string_lossy(),
                            command,
                            command,
                            cmd_args
                        );

                        match Command::new("sh")
                              .arg("-c")
                              .arg(sh_cmd)
                              .output() {
                            Ok(output) => {
                                io::stdout().write_all(&output.stdout).unwrap();
                                io::stderr().write_all(&output.stderr).unwrap();
                            },
                            Err(_) => println!("Failed to execute: {}", command),
                        }
                    },
                    None => {
                        println!("{}: command not found", input_trimmed);
                    }
                }
            }
        }
    }
}
