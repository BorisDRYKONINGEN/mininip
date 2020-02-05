//! A minimalist ini file parser (MinIniP stands for Minimalist Ini Parser). It is written in Rust but I will export its API to the C programming language in
//! order to make various bindings

pub mod datas;
pub mod dump;
pub mod parse;
pub mod errors;


// C bindings
use parse::Parser;
use datas::{Identifier, Value};
use std::collections::HashMap;
use std::panic::catch_unwind;
use errors::{Error, ParseFileError};
use std::os::raw::{c_char, c_int};
use std::ffi::{CString, CStr};

/// Returns a new `Parser` which can be used through FFI
/// . Returns a null pointer in case of error
#[no_mangle]
extern fn mininipNewParser() -> *mut Parser {
    // Since `Box::new` or `Parser::new` may `panic!`, we must use `catch_unwind` because unwinding through FFI is undefined behavior
    catch_unwind(|| {
        Box::into_raw(Box::new(Parser::new()))
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroys a `Parser` created by `mininipNewParser`
/// . I wrote it to handle error cases but you should implicitly destroy it through `mininipGetParserData` in any normal use case
#[no_mangle]
unsafe extern fn mininipDestroyParser(parser: *mut Parser) {
    // There is no reason for `std::mem::drop` or `Box::from_raw` to `panic!` so I assume it is safe to not `catch_unwind`
    std::mem::drop(Box::from_raw(parser));
}

/// Destroys a `Parser` created by `mininipNewParser` and returns the result of `parser.data()` which can be used through FFI
/// . It is useful to retrieve the datas in a parsed file
/// 
/// # Warning
/// The argument `parser` is therefore invalidated and must NOT be used later
#[no_mangle]
unsafe extern fn mininipGetParserData(parser: *mut Parser) -> *mut HashMap<Identifier, Value> {
    // Here, we can `panic!` too
    catch_unwind(|| {
        let parser = Box::from_raw(parser);
        Box::into_raw(Box::new(parser.data()))
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroys the result of `mininipGetParserData`
#[no_mangle]
unsafe extern fn mininipDestroyParserData(data: *mut HashMap<Identifier, Value>) {
    std::mem::drop(Box::from_raw(data));
}

/// A FFI usable error enumeration for reporting error kinds through FFI
/// 
/// # Note
/// This type exists because you use a binding branch of the project. It is recommanded to use `master` unless you want to export the library through FFI
#[repr(C)]
pub enum MininipErrorKind {
    /// No error occured
    NoError = 0,
    /// The file could not be parsed
    ParseError,
    /// An I/O error occured
    IOError,
    /// Any other kind of error occured (may be used for memory allocation errors)
    RuntimeError,
}

impl From<Error> for MininipErrorKind {
    fn from(_err: Error) -> MininipErrorKind {
        MininipErrorKind::ParseError
    }
}

impl From<ParseFileError> for MininipErrorKind {
    fn from(err: ParseFileError) -> MininipErrorKind {
        match err {
            ParseFileError::IOError(_)    => MininipErrorKind::IOError,
            ParseFileError::ParseError(_) => MininipErrorKind::ParseError,
        }
    }
}

/// An FFI usable error structure for reporting error through FFI
/// 
/// # Note
/// This type exists because you use a binding branch of the project. It is recommanded to use `master` unless you want to export the library through FFI
/// 
/// # Warning
/// In some cases, the `msg` field *may* be null. It is especially true if `kind` is `NoError` / `MININIP_NO_ERROR` or `RuntimeError` / `MININIP_RUNTIME_ERROR`
#[repr(C)]
pub struct MininipError {
    pub msg: *mut c_char, // Note that the exposed interface to the C API uses a `const char*`
    pub kind: MininipErrorKind,
}

/// Creates and returns an FFI-friendly error from a Rust-only error
/// 
/// # Warning
/// The returned value must be freed with `mininipDestroyError`
pub fn create_ffi_error<E: Into<MininipErrorKind> + std::error::Error>(err: E) -> MininipError {
    let message = CString::new(format!("{}", err))
        .expect("There should not be any null byte in an error message");

    MininipError {
        msg: message.into_raw(),
        kind: err.into(),
    }
}

/// Destroys an FFI-friendly error
#[no_mangle]
unsafe extern fn mininipDestroyError(err: *mut MininipError) {
    let err = &mut *err;
    if err.msg != std::ptr::null_mut() {
        std::mem::drop(CString::from_raw(err.msg));
    }
}

/// Returns datas from the given file
/// 
/// # Parameters
/// `path` a `*const c_char` / `const char*` which is the path of the file to parse
/// 
/// `datas` a `*mut *mut HashMap<Identifier, Value>` / `MininipData**` a pointer to a FFI handle of the data returned by a parser which will be assigned if the
/// operation succeed (if `mininipParseFile(arg1, arg2).kind` is `NoError` / `MININIP_NO_ERROR`)
/// 
/// # Return value
/// A FFI-compatible error (which can be a `NoError`)
#[no_mangle]
unsafe extern fn mininipParseFile(path: *const c_char, datas: *mut *mut HashMap<Identifier, Value>) -> MininipError {
    // Extracting a valid path from the argument
    let path = CStr::from_ptr(path).to_str();
    let path = match path {
        Ok(val) => val,
        Err(_)  => return MininipError {
            msg: CString::new("Argument is not valid utf-8")
                .expect("There is not any null byte inside the message above")
                .into_raw(),
            kind: MininipErrorKind::RuntimeError,
        },
    };

    catch_unwind(|| {
        match parse::parse_file(path) {
            Ok(val) => {
                let ptr = Box::into_raw(Box::new(val));
                *datas = ptr;

                MininipError {
                    msg: std::ptr::null_mut(),
                    kind: MininipErrorKind::NoError,
                }
            },
            Err(err) => create_ffi_error(err),
        }
    })
    .unwrap_or(MininipError {
        msg: std::ptr::null_mut(),
        kind: MininipErrorKind::RuntimeError,
    })
}

/// An entry in the datas of a parser
/// . It corresponds to a value referenced by an optional section name and a key name
/// 
/// # Warning
/// It must be destroyed with `mininipDestroyEntry`
#[repr(C)]
struct MininipEntry {
    value: MininipValue,
    value_type: MininipType,
}

impl From<Value> for MininipEntry {
    fn from(val: Value) -> MininipEntry {
        match val {
            Value::Raw(s) => MininipEntry {
                value: MininipValue {
                    raw: MininipRawValue {
                        ptr: CString::new(s).unwrap().into_raw(),
                    },
                },
                value_type: MininipType::Raw,
            },
            Value::Str(s) => MininipEntry {
                value: MininipValue {
                    string: MininipStrValue {
                        ptr: CString::new(s).unwrap().into_raw(),
                    },
                },
                value_type: MininipType::Str,
            },
            Value::Int(i) => MininipEntry {
                value: MininipValue { integer: i, },
                value_type: MininipType::Int,
            },
            Value::Float(f) => MininipEntry {
                value: MininipValue { floating: f, },
                value_type: MininipType::Float,
            },
            Value::Bool(b) => MininipEntry {
                value: MininipValue { boolean: b as MininipBoolValue, },
                value_type: MininipType::Bool,
            },
        }
    }
}

/// An FFI-compatible union which references any value supported by Mininip
/// 
/// # See
/// `MininipType` which is the second part of this union. Since an union assumes you know the type of the data, it makes sense to create an FFI-compatible enumeration
/// allowing you to know with which type you are working
#[derive(Clone, Copy)]
#[repr(C)]
union MininipValue {
    raw: MininipRawValue,
    string: MininipStrValue,
    integer: MininipIntValue,
    floating: MininipFloatValue,
    boolean: MininipBoolValue,
}

/// An FFI-compatible enumeration to store the type of a key
/// 
/// # See
/// `MininipValue` which is designed to work together with this type. It stores the value formatted as any type while this one stores the type itself
#[derive(Clone, Copy)]
#[repr(C)]
enum MininipType {
    Raw,
    Str,
    Int,
    Float,
    Bool,
}

/// A raw value according to Mininip (see the documentation of the type Raw)
/// 
/// # Design note
/// It is a struct and not a simple type alias to see easily that this value has been allocated by Mininip and should be destroyed by it
/// 
/// # Warning
/// It must be destroyed by `mininipDestroyRawValue`
#[derive(Clone, Copy)]
#[repr(C)]
struct MininipRawValue {
    ptr: *mut c_char, // note it will be a `const char*` in the C API
}

/// A string according to Mininip (see the documentation of the type Str)
/// 
/// # Design note
/// It is a struct and not a simple type alias to see easily that this value has been allocated by Mininip and should be destroyed by it
/// 
/// # Warning
/// It must be destroyed by `mininipDestroyStrValue`
#[derive(Clone, Copy)]
#[repr(C)]
struct MininipStrValue {
    ptr: *mut c_char, // note it will be a `const char*` in the C API
}

/// An integer according to Mininip (see the documentation of the type Int)
type MininipIntValue = i64;

/// A floating point number according to Mininip
type MininipFloatValue = f64;

/// A boolean according to Mininip
type MininipBoolValue = c_int;
const MININIP_TRUE: c_int = 1;
const MININIP_FALSE: c_int = 0;

/// Returns an entry from a section name and a key name
/// 
/// # Parameters
/// `data` the data returned from the parser
/// 
/// `section` the (optional) section name. Must be null if you want a key from the global scope
/// 
/// `key` the key name
/// 
/// `entry` a pointer to a `MininipEntry` structure
/// 
/// # Return value
/// `true` if the entry exists, `false` otherwise or in case of error (including either any runtime error or an invalid name for section or key)
#[no_mangle]
unsafe extern fn mininipGetEntry(data: *mut HashMap<Identifier, Value>, section: *const c_char, key: *const c_char, entry: *mut MininipEntry) -> MininipBoolValue {
    catch_unwind(|| {
        let section = if section == std::ptr::null() {
            None
        } else {
            let string = match CStr::from_ptr(section).to_str() {
                Ok(val) => val,
                Err(_)  => return MININIP_FALSE,
            };
            Some(String::from(string))
        };
        let key = match CStr::from_ptr(key).to_str() {
            Ok(val) => val,
            Err(_)  => return MININIP_FALSE,
        };
        let key = String::from(key);

        if let Some(val) = &section {
            if !Identifier::is_valid(&val) {
                return MININIP_FALSE;
            }
        }
        if !Identifier::is_valid(&key) {
            return MININIP_FALSE;
        }

        let ident = Identifier::new(section, key);
        let data = &mut *data;
        match data.get(&ident) {
            Some(val) => {
                *entry = MininipEntry::from(val.clone());
                MININIP_TRUE
            },
            None      => MININIP_FALSE,
        }
    })
    .unwrap_or(MININIP_FALSE)
}

/// Destroys a `MininipEntry`
#[no_mangle]
unsafe extern fn mininipDestroyEntry(entry: *mut MininipEntry) {
    let entry = &mut *entry;
    match entry.value_type {
        MininipType::Raw   => std::mem::drop(CString::from_raw(entry.value.raw.ptr)),
        MininipType::Str   => std::mem::drop(CString::from_raw(entry.value.string.ptr)),
        MininipType::Int   => {}, // No ressource to free here
        MininipType::Float => {}, // No ressource to free here
        MininipType::Bool  => {}, // No ressource to free here
        // NOTICE: I could use the `_` pattern here but I wanted an exhaustive match to prevent me from forgetting to update this function when I extend the type
        // system
    }
}


// unit-tests
#[cfg(test)]
mod tests;
