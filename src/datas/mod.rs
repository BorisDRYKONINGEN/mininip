//! The basic datas definitions like `Section` (what a section is) and `State` (the state of a INI script)

use std::fmt::{self, Display, Formatter};
use crate::{parse, dump};

/// The value of a INI variable. May be edited in the future to add new types
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::Str(string)
    }
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Value::Str(string) => string.fmt(formatter),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Str(String::new())
    }
}

impl Value {
    /// Builds a new `Value` from `content`, an INI-formatted string
    /// 
    /// # Return value
    /// `Ok(value)` with `value` as the new object. Note that `value` will always be a `Value::Str` when calling this method
    /// 
    /// `Err(())` when an error occurs while parsing content
    pub fn parse_str(content: &str) -> Result<Value, ()> {
        Ok(Value::Str(parse::parse_str(content)?))
    }

    /// Formats `self` to be dumped in the INI file
    /// 
    /// # Return value
    /// A `String` containing the value of `self` once formatted
    /// 
    /// # See
    /// See `dump_str` for more informations about this format
    /// 
    /// # Note
    /// To keep the same type once backed-up, a `Value::Str` gets two extra `'` around it
    /// 
    /// # Examples
    /// ```
    /// use mininip::datas::Value;
    /// 
    /// let val = Value::from(String::from("très_content=☺ ; the symbol of hapiness"));
    /// let dumped = val.dump();
    /// 
    /// assert_eq!(dumped, "'tr\\x0000e8s_content\\=\\x00263a \\; the symbol of hapiness'"); // Notice the leading and the ending '\''
    /// ```
    pub fn dump(&self) -> String {
        match self {
            Value::Str(string) => format!("'{}'", dump::dump_str(&string)),
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
    /// Panics if either `section` or `name` is an invalid identifier according to `Identifier::is_valid`
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

    /// Returns the name of the variable
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the section of the variable
    pub fn section(&self) -> Option<&str> {
        match &self.section {
            Some(val) => Some(&val),
            None      => None,
        }
    }

    /// Change the name of the variable
    /// 
    /// # Panics
    /// Panics if `name` is invalid
    pub fn change_name(&mut self, name: String) {
        assert!(Identifier::is_valid(&name));

        self.name = name;
    }

    /// Changes the section of the variable
    /// 
    /// # Panics
    /// Panics if `section` is invalid
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
