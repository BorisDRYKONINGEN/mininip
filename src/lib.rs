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
use std::os::raw::c_char;
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


// unit-tests
#[cfg(test)]
mod tests;
