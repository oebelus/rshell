use std::{collections::HashMap, env};

#[derive(Clone)]
pub struct Shell {
    pub environment: HashMap<String, String>,
    pub builtins: Vec<String>
}

impl Shell {
    pub fn new() -> Self {
        let path = match env::var("PATH") {
            Ok(p) => p,
            Err(_) => String::new(),
        };
    
        let home = match env::var("HOME") {
            Ok(p) => p,
            Err(_) => String::new(),
        };

        let mut environment = HashMap::new();
        environment.insert("path".to_string(), path);
        environment.insert("home".to_string(), home);

        Shell {
            environment,
            builtins: vec!["exit", "echo", "type", "pwd", "cd"].iter().map(|x| x.to_string()).collect(),
        }
    }
}