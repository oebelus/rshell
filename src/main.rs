#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let trimmed = input.trim();
    
        if trimmed.len() == 6 && trimmed.starts_with("exit") {
            if trimmed.chars().last().unwrap() == '0' {
                break;
            }
        }

        else if trimmed.starts_with("echo") {
            println!("{}", trimmed[5..].trim());
        }

        else {
            println!("{}: command not found", input.trim());  
        }  
    }
}
