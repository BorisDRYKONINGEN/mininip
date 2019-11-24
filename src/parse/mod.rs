//! Provides tools to parse an INI file

use std::iter::Fuse;

/// Reads a string formatted by `dump_str` and unescapes the escaped characters
/// 
/// # Return value
/// `Ok(string)` with `string` as the result once parsed
/// 
/// `Err(())` This return type may change in the future
/// 
/// # Encoding issues
/// Only allows ASCII because Unicode or other encodings musn't appear in an INI file (except in comments but this function is not intended to parse whole files)
pub fn parse_str(content: &str) -> Result<String, ()> {
    for i in content.chars() {
        if !i.is_ascii() {
            return Err(());
        }
    }

    // new will never be wider than content
    let mut new = String::with_capacity(content.len());

    static FORBIDDEN: [char; 12] = ['\x07', '\x08', '\t', '\r', '\n', '\0', '\\', '\'', '\"', ';', ':', '='];

    for i in TokenIterator::from(content.chars()) {
        let escape = match i {
            Token::Char(c) => {
                if FORBIDDEN.contains(&c) {
                    return Err(());
                }

                new.push(c);
                continue;
            },
            Token::Escape(s) => s,
        };

        match escape.as_str() {
            "\\a"  => new.push('\x07'),
            "\\b"  => new.push('\x08'),
            "\\t"  => new.push('\t'),
            "\\r"  => new.push('\r'),
            "\\n"  => new.push('\n'),
            "\\0"  => new.push('\0'),
            r"\\"  => new.push('\\'),
            "\\'"  => new.push('\''),
            "\\\"" => new.push('\"'),
            "\\;"  => new.push(';'),
            "\\:"  => new.push(':'),
            "\\="  => new.push('='),

            _ if escape.len() == 8 => {
                debug_assert!(escape.starts_with("\\x"));

                let values = &escape[2..];
                let code = match u32::from_str_radix(values, 16) {
                    Ok(val) => val,
                    Err(_)  => return Err(()),
                };
                let character = match std::char::from_u32(code) {
                    Some(val) => val,
                    None      => return Err(()),
                };
                new.push(character);
            },

            _ => return Err(()),
        }
    }

    Ok(new)
}

/// A token which is either a character or an escape sequence
#[derive(PartialEq, Debug)]
enum Token {
    Char(char),
    Escape(String),
}

/// An iterator over the characters of an INI file. These characters are NOT TRUSTED, for example, you may receive a `\é` sequence wich is illegal in INI
/// 
/// Yields `Token`s which can be either a character or an escape sequence.
/// 
/// If an escape sequence is left unfinished, it is returned as is in a `Token::Escape` object, even though it is invalid
struct TokenIterator<T> {
    escape_seq: String,
    iterator: Fuse<T>,
}

impl<T: Iterator> From<T> for TokenIterator<T> {
    fn from(iterator: T) -> TokenIterator<T> {
        TokenIterator {
            iterator: iterator.fuse(),
            escape_seq: String::with_capacity(8),
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for TokenIterator<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            let i = match self.iterator.next() {
                Some(val) => val,

                // When the iterator returns `None`, we return the escape sequence if unfinished or `None` if the text was not escaped
                None if self.escape_seq.is_empty() => return None,
                None => {
                    let mut buf = String::new();
                    std::mem::swap(&mut buf, &mut self.escape_seq);
                    return Some(Token::Escape(buf));
                },
            };

            if !self.escape_seq.is_empty() {
                self.escape_seq.push(i);
            } else if i == '\\' {
                self.escape_seq.push(i);
                continue;
            } else {
                return Some(Token::Char(i));
            }

            if self.escape_seq.starts_with(r"\x") && self.escape_seq.len() < 8 {
                continue;
            }

            let mut buf = String::with_capacity(8);
            std::mem::swap(&mut buf, &mut self.escape_seq);
            return Some(Token::Escape(buf));
        }
    }
}


#[cfg(test)]
mod tests;
