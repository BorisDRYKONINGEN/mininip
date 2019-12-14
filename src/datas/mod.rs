//! The basic datas structures like [`Identifier`](datas/struct.Identifier.html "Identifier") and [`Value`](datas/enum.Value.html "Value")

use std::fmt::{self, Display, Formatter};
use crate::{parse, dump};
use crate::errors::Error;

/// The value of a INI variable
/// 
/// Currently, there is one single type: the `Raw` type. But in the version 1.1.0, the following types will be available
/// - `Raw`: the raw content of the file, no formatted. The only computation is that the escaped characters are unescaped (see [parse_str](../parse/fn.parse_str.html "parse::parse_str") to learn more about escaped characters)
/// - `Str`: a quoted written inside non-escaped quotes like that `"Hello world!"` or that `'Hello world!'`
/// - `Int`: a 64 bytes-sized integer
/// - `Float`: a 64 bytes-sized floating-point number
/// - `Bool`: a boolean
/// 
/// Each type is represented as an enum variant. Since version 1.1.0 or 1.2.0, the deduction of the type when parsing will be automated but you may want to cast it to another, wich will be supported
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Raw(String),
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Value::Raw(string) => string.fmt(formatter),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Raw(String::new())
    }
}

impl Value {
    /// Builds a new [`Value`](enum.Value.html "datas::Value") from `content`, an INI-formatted string
    /// 
    /// # Return value
    /// `Ok(value)` with `value` as the new object. Note that `value` will always be a `Value::Raw` when calling this method until version 1.1.0 or 1.2.0
    /// 
    /// `Err(error)` when an error occurs while parsing `content` with `error` as the error code
    pub fn parse(content: &str) -> Result<Value, Error> {
        Ok(Value::Raw(parse::parse_str(content)?))
    }

    /// Formats `self` to be dumped in an INI file
    /// 
    /// It means that `format!("{}={}", ident, value.dump())` with `ident` as a valid key and `value` a [`Value`](enum.Value.html "Value") can be properly registered and then, parsed as INI
    /// 
    /// # Return value
    /// A `String` containing the value of `self` once formatted
    /// 
    /// # See
    /// See [`dump_str`](fn.dump_str.html "datas::dump_str") for more informations about this format
    /// 
    /// # Note
    /// The type of `self` is backed up in a way preserving the type of `self`
    /// 
    /// - `Raw` is backed up as is, once escaped
    /// - `Str` will be backed up with two quotes `'` or `"` around its value once escaped
    /// - `Int` will be backed up as is
    /// - `Float` will be backed up as is
    /// - `Bool` will be backed up as two different values: `true` and `false`
    /// 
    /// # Examples
    /// ```
    /// use mininip::datas::Value;
    /// 
    /// let val = Value::Raw(String::from("très_content=☺ ; the symbol of hapiness"));
    /// let dumped = val.dump();
    /// 
    /// assert_eq!(dumped, "tr\\x0000e8s_content\\=\\x00263a \\; the symbol of hapiness");
    /// ```
    pub fn dump(&self) -> String {
        match self {
            Value::Raw(string) => format!("{}", dump::dump_str(&string)),
        }
    }
}


/// The identifier of a variable, which is its identity. Of course, this type is `Hash` because it may be used as a key in a `HashMap`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    section: Option<String>,
    name: String,
}

impl Identifier {
    /// Creates an identifier with a valid section name and a valid name
    /// 
    /// # Panics
    /// Panics if either `section` or `name` is an invalid identifier according to [`Identifier::is_valid`](struct.Identifier.html#method.is_valid "datas::Identifier::is_valid")
    pub fn new(section: Option<String>, name: String) -> Identifier {
        if let Some(section) = &section {
            assert!(Identifier::is_valid(section));
        }
        assert!(Identifier::is_valid(&name));

        Identifier {
            section,
            name,
        }
    }

    /// Returns `true` if the given string is a valid identifier and `false` otherwise
    /// 
    /// A valid identifier is defined as a string of alphanumeric characters and underscore `_` **not** starting with a numeric one. All of these characters must be ASCII
    /// 
    /// # Notes
    /// Since the INI file format is not really normalized, this definition may evolve in the future. In fact, I will avoid when possible to make a stronger rule, in order to keep backward compatibility
    /// 
    /// # Examples
    /// ```
    /// use mininip::datas::Identifier;
    /// 
    /// assert!(Identifier::is_valid("identifier"));
    /// assert!(Identifier::is_valid("digits1230"));
    /// assert!(Identifier::is_valid("UPPERCASE_AND_UNDERSCORES"));
    /// assert!(!Identifier::is_valid("123_starts_with_a_digit"));
    /// assert!(!Identifier::is_valid("invalid_characters;!\\~"));
    /// assert!(!Identifier::is_valid("é_is_unicode"));
    pub fn is_valid(ident: &str) -> bool {
        let ident = ident.trim();

        let mut iter = ident.chars();
        match iter.next() {
            // An empty string is not allowed
            None    => return false,

            // The first character must be a letter
            Some(c) => if !c.is_ascii() || !c.is_alphabetic() {
                return false;
            },
        }

        for i in iter {
            // The following ones may be numeric characters
            if !i.is_ascii() || !i.is_alphanumeric() && i != '_' {
                return false;
            }
        }

        true
    }

    /// Returns the name of the variable as a reference
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the section of the variable which may be a named section as `Some(name)` or the "global scope" wich is `None`
    pub fn section(&self) -> Option<&str> {
        match &self.section {
            Some(val) => Some(&val),
            None      => None,
        }
    }

    /// Change the name of the variable
    /// 
    /// # Panics
    /// Panics if `name` is invalid according to [`Identifier::is_valid`](struct.Identifier.html#method.is_valid "datas::Identifier::is_valid")
    pub fn change_name(&mut self, name: String) {
        assert!(Identifier::is_valid(&name));

        self.name = name;
    }

    /// Changes the section of the variable. `section` may be `Some(name)` with `name` as the name of the section of `None` for the "global scope"
    /// 
    /// # Panics
    /// Panics if `section` is invalid according to [`Identifier::is_valid`](struct.Identifier.html#method.is_valid "datas::Identifier::is_valid")
    pub fn change_section(&mut self, section: Option<String>) {
        if let Some(section) = &section {
            assert!(Identifier::is_valid(section));
        }

        self.section = section;
    }
}

impl Display for Identifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if let Some(section) = &self.section {
            formatter.write_str(&section)?;
            formatter.write_str(".")?;
        }

        formatter.write_str(&self.name)
    }
}


#[cfg(test)]
mod tests;
