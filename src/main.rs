#[allow(unused_imports)]

mod shell;
mod file;
mod instruction;

use std::io::{self, Write};
use std::{env::{current_dir, set_current_dir}, process::{exit, Command}};
use file::{executable_exists, is_executable, read_file};

use instruction::Instruction;
use shell::Shell;

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

    let command = instruction.command.trim();

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
        "echo" => println!("{}", instruction.arguments.join(" ").trim()),
        "type" => {
            let command = &instruction.arguments.join("");
            if shell.builtins.contains(&command.as_str()) {
                println!("{} is a shell builtin", command);
                return
            }
            else { 
                executable_exists(&path, command);
            }
        },
        "cat" => {
            let mut cat: Vec<String> = vec![];

            for i in &instruction.arguments {
                cat.push(read_file(&i));
            }

            print!("{}", cat.join(""));
        },
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