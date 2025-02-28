use std::{cell::RefCell, collections::HashSet, io::{self, Write}};

use rustyline::completion::Completer;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

use crate::shfile::find_executables;

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct CommandCompleter {
    pub commands: Vec<String>,
    pub path: String,
    state: RefCell<CompleterState>    
}

struct CompleterState {
    pub tab_count: u8,
    pub last_partial: String,
    pub last_matches: HashSet<String>
} 

impl CommandCompleter {
    pub fn new(commands: Vec<String>, path: String) -> Self {
        Self {
            commands,
            path,
            state: RefCell::new(CompleterState {
                tab_count: 0,
                last_partial: String::new(),
                last_matches: HashSet::new()
            })
        }
    }
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
        let partial = words.last().unwrap_or(&"");

        {
            let mut state = self.state.borrow_mut();
            if state.last_partial != *partial {
                state.tab_count = 0;
                state.last_partial = partial.to_string();
                state.last_matches.clear();
            }
        }

        if words.is_empty() || (words.len() == 1 && !line.ends_with(' ')) {
            let partial = words.last().unwrap_or(&"");

            let mut matches: HashSet<String> = self.commands
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

            let mut matches: Vec<String> = matches.into_iter().collect();
            matches.sort();

            match matches.len() {
                0 => Ok((0, Vec::new())),
                1 => Ok((0, matches.clone().into_iter().collect())),
                _ => {
                    let common_prefix = format!("{} ", common_prefix(matches.clone()));

                    if !common_prefix.is_empty() && matches.contains(&common_prefix){
                        return Ok((0, vec![common_prefix.trim().to_string()]));
                    }

                    let mut state = self.state.borrow_mut();
                    state.tab_count += 1;
                    
                    match state.tab_count {
                        1 => {
                            print!("\x07");
                            io::stdout().flush().unwrap();
                            Ok((0, Vec::new()))
                        },
                        2 => {
                            println!();
                            for m in matches {
                                print!("{} ", m)
                            }
                            println!();
                            print!("$ {}", partial);
                            io::stdout().flush().unwrap(); 
                            Ok((0, Vec::new()))
                        },
                        _ => Ok((0, Vec::new())),
                    }
                }
            }
        } else {
            Ok((pos, Vec::new()))
        }
    }
}

pub fn common_prefix(names: Vec<String>) -> String {
    if names.is_empty() {
        return String::new();
    }

    let mut prefix = names[0].clone();

    for name in names.iter() {
        while !name.starts_with(&prefix) {
            if prefix.is_empty() {
                return String::new();
            }
            prefix.pop();
        }
    }

    prefix
}