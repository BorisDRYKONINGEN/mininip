//! Contains the definition of `Parser`

use std::collections::HashMap;
use crate::datas::{Identifier, Value};
use crate::parse;
use crate::errors::{Error, error_kinds};

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
    pub fn parse_line<'a>(&mut self, line: &'a str) -> Result<(), Error<'a>> {
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
    fn parse_assignment<'a>(&mut self, line: &'a str) -> Result<(), Error<'a>> {
        // Getting the expression of `identifier` in "`identifier` = `value`[;comment]"
        let equal = match line.find('=') {
            Some(index) => index,
            None        => {
                let effective_line = line.trim_start();
                let leading_spaces = line.len() - effective_line.len();

                let end_of_ident = match effective_line.find(char::is_whitespace) {
                    Some(index) => index,
                    None        => effective_line.len(),
                };

                return Err(Error::ExpectedToken(error_kinds::ExpectedToken::new(line, end_of_ident + leading_spaces, String::from("="))));
            }
        };

        let identifier = line[..equal].trim();

        // Getting the expression of `value` in "`identifier` = `value`[;comment]"
        let value = if line.len() == equal + 1 {
            ""
        } else {
            ignore_comment(&line[equal + 1..]).trim()
        };

        if !Identifier::is_valid(identifier) {
            return Err(Error::InvalidIdentifier(error_kinds::InvalidIdentifier::new(line, identifier)));
        }
        let value = parse::parse_str(value)?;

        self.variables.insert(
            Identifier::new(self.cur_section.clone(), String::from(identifier)),
            Value::Str(value),
        );
        Ok(())
    }

    /// Parses a section declaration. A section declaration is of form
    /// 
    /// ```ini
    /// [section];comment
    /// ```
    /// 
    /// # Panics
    /// Panics if line doesn't start with a `[` character, which indicates `line` is not a section declaration but may is a valid INI instruction. In this way, we can't return an error expecting a `[` at the beginning of the line, which doesn't make any sense
    fn parse_section<'a>(&mut self, line: &'a str) -> Result<(), Error<'a>> {
        let initial_line = line;
        let line = line.trim_start();
        let leading_spaces = initial_line.len() - line.len();

        let mut iter = line.char_indices();
        match iter.next() {
            None => panic!("An INI section declaration starts with `[`. {} does not, which means the parser did not call the right function", line),
            Some((_, c)) => if c != '[' {
                panic!("An INI section declaration starts with `[`. {} does not, which means the parser did not call the right function", line);
            },
        }

        let mut end = 0;
        for (n, i) in iter.by_ref() {
            if i == ']' {
                end = n;
                break;
            }
        }

        // end == 0 means that there isn't any ']' while end == 1 means that the section name is empty
        if end == 0 {
            return Err(Error::ExpectedToken(error_kinds::ExpectedToken::new(line, leading_spaces + 1, String::from("]"))));
        } else if end == 1 {
            return Err(Error::ExpectedIdentifier(error_kinds::ExpectedIdentifier::new(line, leading_spaces + 1)));
        }

        let section = &line[1..end];
        if !Identifier::is_valid(section) {
            return Err(Error::InvalidIdentifier(error_kinds::InvalidIdentifier::new(line, section)));
        }

        // Checking integrity: I want to ensure there is no extra character after the section declaration
        // The only ones allowed are the whitespaces and the semicolon (with all the following ones)
        for (n, i) in iter {
            if i == ';' {
                break;
            } else if !i.is_whitespace() {
                return Err(Error::UnexpectedToken(error_kinds::UnexpectedToken::new(line, leading_spaces // The leading spaces ignored
                                                                                         + 2             // The '[' and ']' characters
                                                                                         + section.len() // The identifier
                                                                                         + n)));         // The index after the ']' character
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
fn ignore_comment(line: &str) -> &str {
        let mut end = line.len();
        let mut escaped = false;

        for (n, i) in line.char_indices() {
            assert_ne!(i, '\n', "Found newline character which was not expected");

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
    
    &line[..end]
}


#[cfg(test)]
mod tests;
