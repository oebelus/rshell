use std::fmt::Display;

#[derive(Clone)]
pub struct Redirection {
    pub r_type: RedirType,
    pub path: String
}

// impl Redirection {
//     pub fn new(r_type: RedirType, path: String) -> Self {
//         Redirection {
//             r_type,
//             path: String::from(path)
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub enum RedirType {
    Stdout,
    Stderr,
    None
}

impl Display for RedirType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedirType::Stdout => write!(f, "stdout"),
            RedirType::Stderr => write!(f, "stderr"),
            RedirType::None => write!(f, "none"),
        }
    }
}

pub fn has_redirection(arguments: &Vec<String>) -> bool {
    let redirections = [String::from(">"), String::from("1>"), String::from("2>")];
    arguments.iter().find(|x| redirections.contains(x)).is_some() 
}

pub fn find_redirection(arguments: Vec<String>) -> Result<(Vec<String>, Redirection), bool> {
    let args = arguments.clone();
    let mut redirection = Redirection {
        r_type: RedirType::None,
        path: String::new()
    };

    let mut i = args.len();

    if let Some(index) = args.iter().position(|x| x == ">" || x == "1>" || x == "2>") {
        i = index;
        if index + 1 < args.len() {
            redirection.r_type = match args[index].as_str() {
                ">" => RedirType::Stdout,
                "1>" => RedirType::Stdout,
                "2>" => RedirType::Stderr,
                _ => RedirType::None,
            };

            redirection.path = args[index + 1].clone();
        } else {
            return Err(false);
        }
    }

    Ok((args[..i].to_vec(), redirection))
}
