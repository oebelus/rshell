pub struct Redirection {
    pub r_type: RedirType,
    pub path: String
}

impl Redirection {
    pub fn new(r_type: RedirType, path: String) -> Self {
        Redirection {
            r_type,
            path: String::from(path)
        }
    }
}

pub enum RedirType {
    Stdout,
    Stderr,
    None
}

pub fn find_redirection(arguments: Vec<String>) -> Result<(Vec<String>, Redirection), bool> {
    let args = arguments.clone();
    let mut redirection = Redirection {
        r_type: RedirType::None,
        path: String::new()
    };

    let mut i = args.len();

    if let Some(index) = args.iter().position(|x| x == ">" || x == "1>") {
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
