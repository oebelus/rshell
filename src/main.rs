#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            input if input.starts_with("echo") => println!("{}", input[5..].trim()),
            input if input.starts_with("type") => {
                match input[5..].trim() {
                    "exit" => println!("exit is a shell builtin"),
                    "echo" => println!("echo is a shell builtin"),
                    _ => println!("{}: not found", input[5..].trim())
                }
            }
            "exit 0" => exit(0),
            _ => println!("{}: command not found", input.trim())
        }
    }
}
