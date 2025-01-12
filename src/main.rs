#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = match args.get(1) {
        Some(path) => path,
        None => "",
    };

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        handle_input(&input, &path);
    }
}

fn handle_input(input: &str, path: &str) {
    // let builtins= ["exit", "echo", "type"];

    match input.trim() {
        input if input.starts_with("echo") => println!("{}", input[5..].trim()),
        input if input.starts_with("type") => {
            let command = input[5..].trim();
            executable_exists(path, command);
        }
        "exit 0" => exit(0),
        _ => println!("{}: command not found", input.trim())
    }
}

fn executable_exists(path: &str, command: &str) {
    let directories = path.split(':');

    for directory in directories {
        let full_path = format!("{}/{}", directory, command);
        if std::fs::metadata(&full_path).is_ok() {
            println!("{} is {}", command, full_path);
        }
    }

    println!("{}: command not found", command)
}