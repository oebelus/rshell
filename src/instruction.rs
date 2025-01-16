pub struct Instruction {
    pub command: String,
    pub arguments: Vec<String>
}

impl Instruction {
    pub fn new(input: &str) -> Instruction {
        let mut vector = input.split_whitespace();
        let command = vector.next().unwrap().to_string();
        let arguments = vector;

        Instruction {
            command,
            arguments: arguments.map(str::to_string).collect()
        }
    }
}