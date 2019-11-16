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
            Value::Str(string) => format!("'{}'", dump_str(&string)),
        }
    }
}

/// Formats a `&str` by escaping special characters
/// 
/// # Return value
/// A `String` containing the escaped string
/// 
/// # Why should I format it?
/// The `Display` trait is about displaying a value to the user while `Debug` is for debuging. There is not any trait for dumping a value in a file knowing it can't be backed up in the same way it is displayed, so `escape` does this.
/// 
/// For instance, if `content` is `"a'bc=123;"`, then, `escape` will return `r"a\'bc\=123\;"` because it escapes special characters such as `=`, `'`, `;`, ...
/// 
/// More escaped characters may be found at [Wikipedia](https://en.wikipedia.org/wiki/INI_file#Escape_characters "INI file")
/// 
/// # Examples
/// ```
/// use mininip::datas::dump_str;
/// 
/// assert_eq!(dump_str("a'bc=123;"), r"a\'bc\=123\;");
/// ```
pub fn dump_str(content: &str) -> String {
    let mut new = String::with_capacity(content.len());

    for i in content.chars() {
        match i {
            // Those characters have a special rule to be escaped
            '\\'   => new.push_str(r"\\"),
            '\''   => new.push_str("\\'"),
            '"'    => new.push_str("\\\""),
            '\0'   => new.push_str("\\0"),
            '\x07' => new.push_str("\\a"),
            '\x08' => new.push_str("\\b"),
            '\t'   => new.push_str("\\t"),
            '\r'   => new.push_str("\\r"),
            '\n'   => new.push_str("\\n"),
            ';'    => new.push_str("\\;"),
            '#'    => new.push_str("\\#"),
            '='    => new.push_str("\\="),
            ':'    => new.push_str("\\:"),

            // The ASCII characters are left unchanged
            _ if i.is_ascii() => new.push_str(&format!("{}", i)),

            // The non-ASCII characters are escaped with `\x??????`
            _ => new.push_str(&format!("\\x{:06x}", i as u32)),
        }
    }

    new
}

#[cfg(test)]
mod tests;
