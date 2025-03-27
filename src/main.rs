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
        match input.trim() {
            "exit 0" => break,
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
        if input.trim() == "exit 0" {
            break;
        }
    }
}
