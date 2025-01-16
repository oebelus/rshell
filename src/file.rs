use std::{fs, process::Command};

pub fn executable_exists(path: &str, command: &str) {
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

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't read file")
}