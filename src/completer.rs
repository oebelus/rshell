use rustyline::completion::Completer;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct CommandCompleter {
    pub commands: Vec<String>
}

impl Completer for CommandCompleter {
    type Candidate = String;
    
    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let words: Vec<&str> = line[..pos].split_whitespace().collect();

        if words.is_empty() || (words.len() == 1 && !line.ends_with(' ')) {
            let partial = words.last().unwrap_or(&"");

            let matches = self.commands
                .iter()
                .filter(|cmd| cmd.starts_with(partial))
                .map(|s| format!("{} ", s))
                .collect();

            Ok((0, matches))
        } else {
            Ok((pos, Vec::new()))
        }
    }
}