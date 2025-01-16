#[allow(unused_imports)]

mod shell;

use std::io::{self, Write};
use std::{env::{current_dir, set_current_dir}, fs, process::{exit, Command}};
use shell::Shell;

struct Instruction {
    command: String,
    arguments: Vec<String>
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let mut vector = input.split_whitespace();
        let command = vector.next().unwrap().to_string();
        let arguments = vector;

        Instruction {
            command,
            arguments: arguments.map(str::to_string).collect()
        }
    }
}

fn main() {
    let shell = Shell::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let instruction = Instruction::new(input.trim());

        handle_input(&instruction, &shell);
    }
}

fn handle_input(instruction: &Instruction, shell: &Shell) {
    let home = &shell.environment["home"];
    let path = &shell.environment["path"];

    let command = instruction.command.as_str();

    match command {
        "pwd" => println!("{}", current_dir().unwrap().to_str().unwrap()),
        "cd" => {
            let directory = &instruction.arguments[0];

            match directory.as_str() {
                "~" => 
                    match set_current_dir(home) {
                        Ok(_) => (),
                        Err(_) => println!("Error navigating to home"),
                    },
                _ => 
                    match set_current_dir(directory) {
                        Ok(_) => (),
                        Err(_) => println!("cd: {}: No such file or directory", directory)
                    },
            }
        },
        "echo" => println!("{}", instruction.arguments.join("").trim()),
        "type" => {
            let command = &instruction.arguments.join("");
            if shell.builtins.contains(&command.as_str()) {
                println!("{} is a shell builtin", command);
                return
            }
            else { 
                executable_exists(&path, command);
            }
        }
        "exit" => {
            let argument = instruction.arguments.join("");
            match argument.as_str() {
                "0" => exit(0),
                _ => println!("exit: command not found")
            }
        }
        _ => {
            match is_executable(&path, command) {
                Ok(_) => {
                    Command::new(command)
                        .args(&instruction.arguments)
                        .status()
                        .expect("Failed to execute process");
                }
                Err(_) => eprintln!("{}: command not found", command),
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