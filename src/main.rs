#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::{self, current_dir}, fs, process::{exit, Command}};

fn main() {
    let path = match env::var("PATH") {
        Ok(p) => p,
        Err(_) => String::new(),
    };

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        handle_input(&input, &path);
    }
}

fn handle_input(input: &str, path: &str) {
    let builtins= ["exit", "echo", "type"];
    let splited = input.split_whitespace().collect::<Vec<&str>>();

    match input.trim() {
        "pwd" => println!("{}", current_dir().unwrap().to_str().unwrap()),
        input if input.starts_with("echo") => println!("{}", input[5..].trim()),
        input if input.starts_with("type") => {
            let command = input[5..].trim();
            if builtins.contains(&command) {
                println!("{} is a shell builtin", command);
                return
            }
            else { 
                executable_exists(path, command);
            }
        }
        "exit 0" => exit(0),
        _ => {
            match is_executable(path, splited[0]) {
                Ok(_) => {
                    Command::new(splited[0])
                        .args(&splited[1..])
                        .status()
                        .expect("Failed to execute process");
                }
                Err(_) => eprintln!("{}: command not found", splited[0]),
            }
        }
    }
}

fn executable_exists(path: &str, command: &str) {
    let directories = path.split(':');

    for directory in directories {
        let full_path = format!("{}/{}", directory, command);
        if std::fs::metadata(&full_path).is_ok() {
            println!("{} is {}", command, full_path);
            return;
        }
    }

    println!("{}: not found", command)
}

fn is_executable(path: &str, command: &str) ->Result<String, bool> {
    let directories = path.split(':');

    for directory in directories {
        let full_path = format!("{}/{}", directory, command);
        
        if fs::metadata(&full_path).is_ok() {
            if Command::new(&full_path).output().is_ok() {
                return Ok(full_path)
            }
        }
    }
    Err(false)
}