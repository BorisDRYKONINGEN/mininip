//! This module contains several error error types and their implementations

use std::error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error<'a> {
    ExpectedIdentifier(error_kinds::ExpectedIdentifier<'a>),
    ExpectedToken(error_kinds::ExpectedToken<'a>),
    ExpectedEscape(error_kinds::ExpectedEscape<'a>),
    UnexpectedToken(error_kinds::UnexpectedToken<'a>),
}

impl error::Error for Error<'_> {}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ExpectedIdentifier(err) => write!(f, "{}", err),
            Error::ExpectedToken(err)      => write!(f, "{}", err),
            Error::ExpectedEscape(err)     => write!(f, "{}", err),
            Error::UnexpectedToken(err)    => write!(f, "{}", err),
        }
    }
}

/// Contains all the error types used in `Error`'s variants
pub mod error_kinds {
    use std::error;
    use std::fmt::{self, Display};

    #[derive(Debug)]
    pub struct ExpectedIdentifier<'a> {
        index: usize,
        line: &'a str,
    }

    impl error::Error for ExpectedIdentifier<'_> {}

    impl Display for ExpectedIdentifier<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Expected identifier {}{{here}}{}", &self.line[..self.index], &self.line[self.index..])
        }
    }

    impl<'a> ExpectedIdentifier<'a> {
        /// Creates a new `ExpectedIdentifier` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured. Should be complete
        /// 
        /// `index`: the index where the identifier is expected
        /// 
        /// # Panics
        /// Panics if index is too big
        pub fn new(line: &'a str, index: usize) -> ExpectedIdentifier<'a> {
            assert!(line.len() > index, "`index` must be a valid index in `line`");

            ExpectedIdentifier {
                line,
                index,
            }
        }
    }

    #[derive(Debug)]
    pub struct ExpectedToken<'a> {
        index: usize,
        line: &'a str,
        tokens: String,
    }

    impl error::Error for ExpectedToken<'_> {}

    impl Display for ExpectedToken<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Expected {} {}{{here}}{}", self.tokens, &self.line[..self.index], &self.line[self.index..])
        }
    }

    impl<'a> ExpectedToken<'a> {
        /// Creates a new `ExpectedToken` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured. Should be complete
        /// 
        /// `index`: the index where the token is expected
        /// 
        /// `tokens`: the possible tokens. There is no rule to format it, you just should be aware this will be printed directly to the end user
        /// 
        /// # Panics
        /// Panics if `index` is too big
        pub fn new(line: &'a str, index: usize, tokens: String) -> ExpectedToken<'a> {
            assert!(line.len() > index, "`index` must be a valid index");

            ExpectedToken {
                line,
                index,
                tokens,
            }
        }
    }

    #[derive(Debug)]
    pub struct ExpectedEscape<'a> {
        index: usize,
        line: &'a str,
        replace: String,
        token: char,
    }

    impl error::Error for ExpectedEscape<'_> {}

    impl Display for ExpectedEscape<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Expected escape sequence {} instead of {} in {}{{here}}{}", 
                       self.replace,
                       self.token,
                       &self.line[..self.index],
                       &self.line[self.index + self.token.len_utf8()..])
        }
    }

    impl<'a> ExpectedEscape<'a> {
        /// Creates a new `ExpectedEscape` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured
        /// 
        /// `index`: the index of the error
        /// 
        /// `replace`: the escape sequence which should be used instead
        /// 
        /// # Panics
        /// Panics if `index` is too big or is at an invalid position
        pub fn new(line: &'a str, index: usize, replace: String) -> ExpectedEscape<'a> {
            ExpectedEscape {
                line,
                token: super::nth_char(line, index),
                replace,
                index,
            }
        }
    }

    #[derive(Debug)]
    pub struct UnexpectedToken<'a> {
        index: usize,
        line: &'a str,
        token: char,
    }

    impl error::Error for UnexpectedToken<'_> {}

    impl Display for UnexpectedToken<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Unexpected token {} {}{{here}}",
                       self.token,
                       &self.line[..self.index])
        }
    }

    impl<'a> UnexpectedToken<'a> {
        /// Creates a new `UnexpectedToken` error
        /// 
        /// # Parameters
        /// `line`: the line where the error occured
        /// 
        /// `index`: the index where a token was not expected
        /// 
        /// # Panics
        /// Panics if `index` is too big or is at an invalid position
        pub fn new(line: &'a str, index: usize) -> UnexpectedToken<'a> {
            UnexpectedToken {
                line,
                index,
                token: super::nth_char(line, index),
            }
        }
    }
}

/// Returns the character at the `index`th index (`index` is in bytes) in `string`
/// 
/// # Panics
/// Panics if `index` is out of range or between two bytes of the same character
fn nth_char(string: &str, index: usize) -> char {
    assert!(string.len() > index, "`index` must be a valid index in `string`");

    let mut token = '\0';
    let mut last_n = 0;

    for (n, i) in string.char_indices() {
        last_n = n;

        if n == index {
            token = i;
            break;
        } else if n > index {
            panic!("`index` is not a valid index in `string` (`index` = {:?}, `string` = {:?})", index, string);
        }
    }

    assert_eq!(last_n, index, "`index` is not a valid index in `string` (`index` = {:?}, `string` = {:?})", index, string);

    token
}


#[cfg(test)]
mod tests;
