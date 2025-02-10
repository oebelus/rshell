mod shell;
mod shfile;
mod instruction;
mod redirection;
mod sherror;

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::{env::{current_dir, set_current_dir}, process::{exit, Command}};
use shfile::{executable_exists, is_executable};
use std::path::PathBuf;
use sherror::ShellError;
use sherror::get_error_message;
use instruction::Output;

use instruction::Instruction;
use redirection::{find_redirection, RedirType, Redirection};
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

                let output = handle_input(command, arguments.clone(), shell)
                .map_err(|err| get_error_message(&err).unwrap().to_string());

                redirect_output(redirection.clone(), output).unwrap();
            },
            Err(_) => println!("Redirection in wrong format."),
        }
    } else {
        match handle_input(command, arguments.clone(), shell) {
            Ok(output) => {
                if output.to_string().is_empty() {
                    print!("");
                } else {
                    println!("{}", output.to_string().trim());
                }
            },
            Err(err) => println!("{}", get_error_message(&err).unwrap()),
        }
    }
}

fn handle_input(command: &str, arguments: Vec<String>, shell: &Shell) -> Result<Output, ShellError> {
    let home = &shell.environment["home"];
    let path = &shell.environment["path"];

    match command {
        "pwd" => current_dir()
            .map_err(ShellError::from)
            .and_then(|path| {
                path.to_str()
                    .ok_or_else(|| ShellError::ExecutionError("Invalid path encoding".to_string()))
                    .map(|s| Output::String(s.to_string()))
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
                .map(|_| Output::String(String::new()))
            
        },

        "echo" => Ok(Output::String(arguments.join(" ").trim().to_string())),

        "type" => {
            let command = &&arguments.join("");
            if shell.builtins.contains(&command.as_str()) {
                Ok(Output::String(String::from(format!("{} is a shell builtin", command))))
            }
            else { 
                match executable_exists(&path, command) {
                    Ok(x) => Ok(Output::String(x)),
                    Err(x) => Err(x)
                }
            }
        },

        "cat" => {
            let mut vec_stdout: Vec<String> = vec![];
            let mut vec_stderr: Vec<String> = vec![];
            
            for i in &arguments {
                let c = fs::read_to_string(i).map_err(|_| ShellError::FileNotFound(format!("cat: {}: No such file or directory", i)));
                
                match c {
                    Ok(x) => vec_stdout.push(x),
                    Err(e) => vec_stderr.push(get_error_message(&e).unwrap().to_string()),
                }
            }

            Ok(Output::StdOutErr(vec_stdout.join(""), vec_stderr.join("\n")))
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
                        Ok(Output::String(String::from_utf8(output.stdout).expect("Command executed successfully")))
                    } else {
                        Err(ShellError::ExecutionError(String::from_utf8(output.stderr).expect("Command failed")))
                    }
                }
                Err(_) => Err(ShellError::FileNotFound(String::from(format!("{}: command not found", command)))),
            }
        }
    }
}

fn redirect_output(redirection: Redirection, content: Result<Output, String>) -> Result<(), String> {
    let path = Path::new(&redirection.path);

    let mut file = File::create(path).unwrap();

    match content {
        Ok(Output::String(x)) => {
            if redirection.r_type == RedirType::Stdout {
                file.write_all(x.as_bytes()).map_err(|e| e.to_string())?;
            } else {
                if x.is_empty() {
                    print!("");
                } else {
                    println!("{}", x.to_string().trim());
                }
            }
        },
        Ok(Output::StdOutErr(stdout, stderr)) => {
            match redirection.r_type {
                RedirType::Stdout => {
                    if !stderr.is_empty() {
                        println!("{}", stderr.trim());
                    }

                    file.write_all(stdout.as_bytes()).map_err(|e| e.to_string())?;
                },
                RedirType::Stderr => {
                    file.write_all(stderr.as_bytes()).map_err(|e| e.to_string())?;

                    if !stdout.is_empty() {
                        println!("{}", stdout.trim());
                    }
                },
                RedirType::None => {
                    if !stdout.is_empty() {
                        println!("{}", stdout.trim());
                    }
                    if !stderr.is_empty() {
                        eprintln!("{}", stderr.trim());
                    }
                },
            }
        }
        Err(e) => {
            if redirection.r_type == RedirType::Stderr {
                file.write_all(e.as_bytes()).map_err(|e| e.to_string())?;
            } else {
                if e.is_empty() {
                    print!("");
                } else {
                    println!("{}",e.trim());
                }
            }
        },
    }

    Ok(())
}
