//! Contains the definition of `Parser`

use std::collections::HashMap;
use crate::datas::{Identifier, Value};
use crate::parse;

/// A parser with a local state. Use it by passing it the text to parse line after line
#[derive(Debug, Clone)]
pub struct Parser {
    variables: HashMap<Identifier, Value>,
    cur_section: Option<String>,
}

impl Parser {
    /// Creates a new `Parser`, which didn't parsed any line
    pub fn new() -> Parser {
        Parser {
            variables: HashMap::new(),
            cur_section: None,
        }
    }

    /// Consumes the parser and returns its data which is an `HashMap<Identifier, Value>` linking an identifier to its value
    pub fn data(self) -> HashMap<Identifier, Value> {
        self.variables
    }

    /// Parses a line
    /// 
    /// # Parameters
    /// `line` the line to parse
    /// 
    /// # Return value
    /// `Ok(())` in case of success
    /// 
    /// `Err(())` in case of error
    pub fn parse_line(&mut self, line: &str) -> Result<(), ()> {
        Err(())
    }

    /// Parses an assignment ligne. An assignment is of form
    /// 
    /// ```ini
    /// identifier=value;comment
    /// ```
    fn parse_assignment(&mut self, line: &str) -> Result<(), ()> {
        // Getting the expression of `identifier` in "`identifier` = `value`[;comment]"
        let equal = match line.find('=') {
            Some(index) => index,
            None        => return Err(()),
        };

        let identifier = line[..equal].trim();

        // Getting the expression of `value` in "`identifier` = `value`[;comment]"
        let value = if line.len() == equal + 1 {
            ""
        } else {
            ignore_comment(&line[equal + 1..])?.trim()
        };

        let identifier = parse::parse_str(identifier)?;
        if !Identifier::is_valid(&identifier) {
            return Err(())
        }
        let value = parse::parse_str(value)?;

        self.variables.insert(
            Identifier::new(self.cur_section.clone(), identifier),
            Value::Str(value),
        );
        Ok(())
    }
}

/// Returns a subslice of the given slice which is comment-free (stopped at the first non-escaped semicolon ';'). `line` should be a single line
/// 
/// # Panics
/// Panics if a newline character '\n' is found in line. Note that once the non-escaped semicolon is found, the rest may be not read
fn ignore_comment(line: &str) -> Result<&str, ()> {
        let mut end = line.len();
        let mut escaped = false;

        for (n, i) in line.char_indices() {
            if i == '\n' {
                return Err(());
            }

            if escaped {
                escaped = false;

                continue;
            }

            if i == '\\' {
                escaped = true;
            } else if i == ';' {
                end = n;
                break;
            }
        }
    
    Ok(&line[..end])
}


#[cfg(test)]
mod tests;
