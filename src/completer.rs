use rustyline::completion::Completer;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

use crate::shfile::find_executables;

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct CommandCompleter {
    pub commands: Vec<String>,
    pub path: String
}

impl Completer for CommandCompleter {
    type Candidate = String;
    
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let words: Vec<&str> = line[..pos].split_whitespace().collect();

        if words.is_empty() || (words.len() == 1 && !line.ends_with(' ')) {
            let partial = words.last().unwrap_or(&"");

            let mut matches: Vec<String> = self.commands
                .iter()
                .filter(|cmd| cmd.starts_with(partial))
                .map(|s| format!("{} ", s))
                .collect();

            if matches.len() == 0 {
                matches = find_executables(&self.path, partial)
                    .iter()
                    .filter(|cmd| cmd.starts_with(partial))
                    .map(|s| format!("{} ", s))
                    .collect();
            }

            Ok((0, matches))
        } else {
            Ok((pos, Vec::new()))
        }
    }
}