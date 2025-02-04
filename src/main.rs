mod shell;
mod shfile;
mod instruction;
mod redirection;
mod sherror;

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::{env::{current_dir, set_current_dir}, process::{exit, Command}};
use anyhow::Error;
use shfile::{executable_exists, is_executable};
use std::path::PathBuf;
use sherror::ShellError;
use sherror::get_error_message;

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

                match handle_input(command, arguments.clone(), shell) {
                    Ok(output) => {
                        match redirect_output(redirection, &output) {
                            Ok(_) => return,
                            Err(_) => (),
                        }
                    },
                    Err(err) => {
                        match redirect_output(redirection, get_error_message(&err).unwrap()) {
                            Ok(_) => return,
                            Err(_) => (),
                        }
                    },
                }
            },
            Err(_) => println!("Redirection in wrong format."),
        }
    } else {
        match handle_input(command, arguments.clone(), shell) {
            Ok(output) => {
                if output.is_empty() {
                    print!("");
                } else {
                    println!("{}", output.trim());
                }
            },
            Err(_) => (),
        }
    }
}

fn handle_input(command: &str, arguments: Vec<String>, shell: &Shell) -> Result<String, ShellError> {
    let home = &shell.environment["home"];
    let path = &shell.environment["path"];

    match command {
        "pwd" => current_dir()
            .map_err(ShellError::from)
            .and_then(|path| {
                path.to_str()
                    .ok_or_else(|| ShellError::ExecutionError("Invalid path encoding".to_string()))
                    .map(|s| s.to_string())
        }),

        "cd" => {
            let directory = &arguments[0];

            if directory.is_empty() {
                return Err(ShellError::InvalidArgument("No directory specified".to_string()));
            }

            let path = match directory.as_str() {
                "~" => PathBuf::from(home),
                dir => PathBuf::from(dir)
            };

            set_current_dir(path)
                .map_err(|_| ShellError::FileNotFound(format!("cd: {}: No such file or directory", directory)))
                .map(|_| String::new())
            
        },

        "echo" => Ok(arguments.join(" ").trim().to_string()),

        "type" => {
            let command = &&arguments.join("");
            if shell.builtins.contains(&command.as_str()) {
                Ok(String::from(format!("{} is a shell builtin", command)))
            }
            else { 
                match executable_exists(&path, command) {
                    Ok(x) => Ok(x),
                    Err(x) => Err(x)
                }
            }
        },

        "cat" => {
            let mut vec = vec![];
            
            for i in &arguments {
                match fs::read_to_string(i) {
                    Ok(content) => Ok(vec.push(content)),
                    Err(_) => Err(ShellError::FileNotFound(format!("cat: {}: No such file or directory", i))),
                };
            }

            Ok(String::from(format!("{}", vec.join(""))))
        },

        "exit" => {
            let argument = &arguments.join("");
            match argument.as_str() {
                "0" => exit(0),
                _ => Err(ShellError::InvalidArgument(String::from(format!("exit: command not found"))))
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
                        Ok(String::from_utf8(output.stdout).expect("Command executed successfully"))
                    } else {
                        Err(ShellError::ExecutionError(String::from_utf8(output.stderr).expect("Command failed")))
                    }
                }
                Err(_) => Err(ShellError::FileNotFound(String::from(format!("{}: command not found", command)))),
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