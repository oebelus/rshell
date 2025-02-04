#[allow(unused_imports)]

mod shell;
mod file;
mod instruction;
mod redirection;

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::{env::{current_dir, set_current_dir}, process::{exit, Command}};
use anyhow::Error;
use file::{executable_exists, is_executable};

use instruction::Instruction;
use redirection::{find_redirection, Redirection};
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

        execute_cmd(&instruction, &shell);
    }
}

fn execute_cmd(instruction: &Instruction, shell: &Shell) {
    let command = instruction.command.trim();
    let mut arguments = instruction.arguments.clone();

    if instruction.redirection {
        match find_redirection(arguments.clone()) {
            Ok((x, y)) => {
                arguments = x;
                let redirection = y;

                let output = handle_input(command, arguments.clone(), shell);

                match redirect_output(redirection, &output) {
                    Ok(_) => return,
                    Err(_) => (),
                }
            },
            Err(_) => println!("Redirection in wrong format."),
        }
    } else {
        let output = handle_input(command, arguments.clone(), shell);

        if output.is_empty() {
            print!("");
        } else {
            println!("{}", output.trim());
        }
    }
}

fn handle_input(command: &str, arguments: Vec<String>, shell: &Shell) -> String {
    let home = &shell.environment["home"];
    let path = &shell.environment["path"];

    match command {
        "pwd" => current_dir().unwrap().to_str().unwrap().to_string(),
        "cd" => {
            let directory = &arguments[0];

            match directory.as_str() {
                "~" => 
                    match set_current_dir(home) {
                        Ok(_) => String::from(""),
                        Err(_) => String::from("Error navigating to home"),
                    },
                _ => 
                    match set_current_dir(directory) {
                        Ok(_) => String::from(""),
                        Err(_) => String::from(format!("cd: {}: No such file or directory", directory))
                    },
            }
        },
        "echo" => String::from(format!("{}", &arguments.join(" ").trim())),
        "type" => {
            let command = &&arguments.join("");
            if shell.builtins.contains(&command.as_str()) {
                String::from(format!("{} is a shell builtin", command))
            }
            else { 
                executable_exists(&path, command)
            }
        },
        "cat" => {
            let mut vec = vec![];
            
            for i in &arguments {
                match fs::read_to_string(i) {
                    Ok(content) => vec.push(content),
                    Err(_) => println!("cat: {}: No such file or directory", i),
                };
            }

            String::from(format!("{}", vec.join("")))
        },
        "exit" => {
            let argument = &arguments.join("");
            match argument.as_str() {
                "0" => exit(0),
                _ => String::from(format!("exit: command not found"))
            }
        }
        _ => {
            match is_executable(&path, command) {
                Ok(_) => {
                    let output = Command::new(command)
                        .args(&arguments)
                        .output()
                        .expect("Failed to execute process");

                    if output.status.success() {
                        String::from_utf8(output.stdout).expect("Command executed successfully")
                    } else {
                        String::from_utf8(output.stderr).expect("Command failed")
                    }
                }
                Err(_) => String::from(format!("{}: command not found", command)),
            }
        }
    }
}

fn redirect_output(redirection: Redirection, content: &str) -> Result<(), Error> {
    let path = Path::new(&redirection.path);

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}