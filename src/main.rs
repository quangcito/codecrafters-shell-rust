#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let env_path = std::env::vars_os()
        .find(|v| "PATH".eq(&v.0))
        .map(|ev| ev.1.into_string().unwrap());
    let b: String;
    let ep = if env_path.is_some() {
        b = env_path.unwrap();
        Some(b.split(':').map(|p| Path::new(p)).collect::<Vec<_>>())
    } else {
        None::<Vec<&Path>>
    };
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim_end();
        match trimmed_input {
            s if s.starts_with("echo ") => {
                println!("{}", trimmed_input.split_at(5).1);
            }
            s if s.starts_with("type ") => {
                let arg = s.split_at(5).1;
                match arg {
                    "echo" | "exit" | "type" => println!("{arg} is a shell builtin"),
                    _ => match ep {
                        Some(ref e) => {
                            if let Some(dir) = e.iter().find(|t| t.join(arg).exists()) {
                                println!("{} is {}/{}", arg, dir.to_str().unwrap(), arg);
                            } else {
                                println!("{arg}: not found");
                            }
                        }
                        None => println!("{arg}: not found"),
                    },
                }
            }
            "exit 0" => break,
            line => {
                let (cmd, args) = if line.contains(' ') {
                    line.split_once(' ').unwrap()
                } else {
                    (line, "")
                };
                match ep {
                    Some(ref e) => {
                        if let Some(_dir) = e.iter().find(|t| t.join(cmd).exists()) {
                            let output = Command::new(cmd)
                                .args(args.split_whitespace())  // Changed to split_whitespace for better handling
                                .output()
                                .unwrap();

                            // Print stdout only once
                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        } else {
                            println!("{cmd}: command not found");
                        }
                    }
                    None => println!("{cmd}: command not found"),
                }
            }
        }
    }
}
