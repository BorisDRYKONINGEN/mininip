//! The basic datas definitions like `Section` (what a section is) and `State` (the state of a INI script)

use std::fmt::{self, Display, Formatter};

/// The value of a INI variable. May be edited in the future to add new types
#[derive(Debug, Clone)]
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
            Value::Str(string) => format!("'{}'", crate::dump::dump_str(&string)),
        }
    }
}


#[cfg(test)]
mod tests;
