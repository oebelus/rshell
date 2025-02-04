use std::fmt;

pub enum ShellError {
    IoError(std::io::Error),
    FileNotFound(String),
    InvalidArgument(String),
    ExecutionError(String),
    ExecutableNotFound(String)
}

impl From<std::io::Error> for ShellError {
    fn from(error: std::io::Error) -> Self {
        ShellError::IoError(error)
    }
}

pub fn get_error_message(err: &ShellError) -> Option<&str> {
    match err {
        ShellError::FileNotFound(msg) 
        | ShellError::InvalidArgument(msg) 
        | ShellError::ExecutionError(msg) 
        | ShellError::ExecutableNotFound(msg) => Some(msg),
        
        ShellError::IoError(_) => None,
    }
}