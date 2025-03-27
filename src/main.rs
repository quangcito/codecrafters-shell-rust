#[allow(unused_imports)]
use std::io::{self, Write};

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
                    _ => println!("{}: not found", args),
                },
                &_ => println!("{}: command not found", input.trim()),
            },
            None => println!("{}: command not found", input.trim()),
        }
    }
}
