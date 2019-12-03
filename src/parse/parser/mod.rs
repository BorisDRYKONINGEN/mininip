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
        let effective_line = line.trim_start();

        match effective_line.chars().next() {
            None | Some(';')    => Ok(()),
            Some(c) if c == '[' => self.parse_section(effective_line),
            Some(_)             => self.parse_assignment(effective_line),
        }
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

    /// Parses a section declaration. A section declaration is of form
    /// 
    /// ```ini
    /// [section];comment
    /// ```
    fn parse_section(&mut self, line: &str) -> Result<(), ()> {
        let line = line.trim_start();

        let mut iter = line.char_indices();
        match iter.next() {
            None => return Err(()),
            Some((_, c)) => if c != '[' {
                return Err(());
            },
        }

        let mut end = 0;
        for (n, i) in iter.by_ref() {
            if i == ']' {
                end = n;
                break;
            }
        }

        // end < 1 means that iter was never iterated while end < 2 means that the section name is empty
        if end < 2 {
            return Err(());
        }

        let section = &line[1..end];
        if !Identifier::is_valid(section) {
            return Err(());
        }

        // Checking integrity: I want to ensure there is no extra character after the section declaration
        // The only ones allowed are the whitespaces and the semicolon (with all the following ones)
        for (_, i) in iter {
            if i == ';' {
                break;
            } else if !i.is_whitespace() {
                return Err(());
            }
        }

        self.cur_section = Some(String::from(section));
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
