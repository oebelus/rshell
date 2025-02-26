use crate::sherror::ShellError;

use std::{fs, process::Command};

pub fn executable_exists(path: &str, command: &str) -> Result<String, ShellError> {
    let directories = path.split(':');

    for directory in directories {
        let full_path = format!("{}/{}", directory, command);
        if std::fs::metadata(&full_path).is_ok() {
            return Ok(String::from(format!("{} is {}", command, full_path)));
        }
    }

    Err(ShellError::ExecutableNotFound(format!("{}: not found", command)))
}

pub fn find_executables(path: &str, partial: &str) -> Vec<String> {
    let executables: Vec<String> = list_content(path);
    let mut exec_completion = vec![];

    for executable in executables {
        if executable.starts_with(partial) {
            exec_completion.push(executable);
        }
    }

    exec_completion
}

pub fn is_executable(path: &str, command: &str) -> Result<String, bool> {
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

pub fn list_content(path: &str) -> Vec<String> {
    let directories = path.split(':');

    let output = Command::new("ls")
        .args(directories)
        .output()
        .expect("IO Error");
    
    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .map(|s| s.to_string())
        .collect()
}