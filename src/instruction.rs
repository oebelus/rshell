pub struct Instruction {
    pub command: String,
    pub arguments: Vec<String>
}

impl Instruction {
    pub fn new(input: &str) -> Instruction {
        let vector = parse_command(input);
        let command = vector.get(0).unwrap().to_string();
        let arguments = vector[1..].iter().filter(|s| !s.trim().is_empty()).cloned().collect();

        Instruction {
            command,
            arguments
        }
    }
}

fn parse_command(input: &str) -> Vec<String> {
    let length = input.len();
    let mut i = 0;
    let mut buffer = String::from(""); 
    
    let mut result: Vec<String> = vec![];

    while i < length {
        match input.chars().nth(i) {
            Some(x) => match x {
                ' ' | '\t' | '\n' | '\r' => {
                    if !buffer.is_empty() {
                        result.push(buffer.clone());
                        buffer.clear();
                    }
                }
                '\'' => {
                    i += 1;
                    while input.chars().nth(i) != Some('\'') && i < length {
                        buffer.push(input.chars().nth(i).unwrap());
                        i += 1;
                    }
                    
                    if !buffer.is_empty() && input.chars().nth(i) != Some('\'') {
                        result.push(buffer.clone());
                        buffer.clear();
                    }
                }
                '"' => {
                    let escape = ['\\', '$', '"', '\n'];

                    i += 1;
                    while input.chars().nth(i) != Some('"') && i < length {
                        if input.chars().nth(i) == Some('\\') && i + 1 < length && escape.contains(&input.chars().nth(i+1).unwrap()) {
                            i += 1;
                            buffer.push(input.chars().nth(i).expect("Not Found"));
                            i += 1;
                        } else {
                            buffer.push(input.chars().nth(i).expect("Not Found"));
                            i += 1;
                        }
                    }
                    
                }
                '\\' => {
                    if i < length - 1 {
                        buffer.push(input.chars().nth(i + 1).unwrap());
                    }
                    i += 1;
                }
                _ => {
                    buffer.push(x);
                }
            },
            None => break
        }
        i += 1;
    }

    if !buffer.is_empty() {
        result.push(buffer);
    }

    result
}