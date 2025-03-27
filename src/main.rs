#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input = String::new();


    loop {
        print!("$ ");
        stdout.flush().unwrap();

        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            break;
        }
        println!("{}: command not found", input.trim());
        input.clear();
    }
}
