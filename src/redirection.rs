use std::fmt::Display;

#[derive(Clone)]
pub struct Redirection {
    pub r_type: RedirType,
    pub path: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum RedirType {
    Stdout(RedirOp),
    Stderr(RedirOp),
    None
}

#[derive(Debug, PartialEq, Clone)]
pub enum RedirOp {
    Write,
    Append
}

impl Display for RedirOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedirOp::Write => write!(f, "Write"),
            RedirOp::Append => write!(f, "Append"),
        }
    }
}

impl Display for RedirType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedirType::Stdout(redir_op) => write!(f, "Stdout: {}", redir_op),
            RedirType::Stderr(redir_op) => write!(f, "Stderr: {}", redir_op),
            RedirType::None => write!(f, "none"),
        }
    }
}

impl RedirType {
    pub fn get_redir_op(self) -> Option<RedirOp> {
        match self {
            RedirType::Stdout(op) | RedirType::Stderr(op) => Some(op),
            RedirType::None => None,
        }
    }
}

pub fn has_redirection(arguments: &Vec<String>) -> bool {
    let redirections = [String::from(">"), String::from(">>"), String::from("1>"), String::from("1>>"), String::from("2>"), String::from("2>>")];
    arguments.iter().find(|x| redirections.contains(x)).is_some() 
}

pub fn find_redirection(arguments: Vec<String>) -> Result<(Vec<String>, Redirection), bool> {
    let args = arguments.clone();
    let mut redirection = Redirection {
        r_type: RedirType::None,
        path: String::new()
    };

    let mut i = args.len();

    if let Some(index) = args.iter().position(|x| x == ">" || x == ">>" || x == "1>" || x == "1>>" || x == "2>" || x == "2>>" ) {
        i = index;
        if index + 1 < args.len() {
            redirection.r_type = match args[index].as_str() {
                ">>" => RedirType::Stdout(RedirOp::Append),
                ">" => RedirType::Stdout(RedirOp::Write),
                "1>>" => RedirType::Stdout(RedirOp::Append),
                "1>" => RedirType::Stdout(RedirOp::Write),
                "2>>" => RedirType::Stderr(RedirOp::Append),
                "2>" => RedirType::Stderr(RedirOp::Write),
                _ => RedirType::None,
            };

            redirection.path = args[index + 1].clone();
        } else {
            return Err(false);
        }
    }

    Ok((args[..i].to_vec(), redirection))
}
