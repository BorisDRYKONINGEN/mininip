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

/// Exports an arbitrary through FFI
/// 
/// # Parameters
/// `obj` the object to export
/// 
/// # Return value
/// A raw pointer to `obj` which has been moved on the heap
/// 
/// # See
/// `ffi_destroy` to destroy the pointer returned properly
pub fn ffi_export<T>(obj: T) -> *mut T {
    let obj = Box::new(obj);
    Box::into_raw(obj)
}

/// Destroys an object exported with `ffi_export`
/// 
/// # Parameters
/// `ptr` a pointer to the object to destroy
pub unsafe fn ffi_destroy<T>(ptr: *mut T) {
    std::mem::drop(Box::from_raw(ptr));
}

/// Exports a string through FFI
/// 
/// # Parameters
/// `string` the string to export
/// 
/// # Return value
/// A raw pointer to a new string which has been copied on the heap
/// 
/// # Panics
/// Panics if `string` contains a null character `U+0000`
/// 
/// # See
/// `ffi_destroy_str` to destroy the pointer returned properly
pub fn ffi_export_str(string: &str) -> *mut c_char {
    CString::new(string).unwrap().into_raw()
}

/// Destroys a string previously exported with `ffi_export_str`
/// 
/// # Parameters
/// `ptr` a pointer to the object to destroy
pub unsafe fn ffi_destroy_str(ptr: *mut c_char) {
    std::mem::drop(CString::from_raw(ptr));
}

/// Casts an FFI string into a non-owned Rust one *without* invalidating the pointer
/// 
/// # Parameters
/// `ptr` a pointer to an FFI string
/// 
/// # Return value
/// A string slice to the decoded text in case of success
/// 
/// An `Utf8Error` in case of error
pub unsafe fn ffi_decode_str(ptr: *const c_char) -> Result<&'static str, std::str::Utf8Error> {
    CStr::from_ptr(ptr).to_str()
}

/// Destroys any string allocated by Mininip
/// 
/// # Parameters
/// `string` the string to free. Must be allocated by Mininip
#[no_mangle]
unsafe extern fn mininipDestroyString(string: *mut c_char) {
    ffi_destroy_str(string);
}

/// Returns a new `Parser` which can be used through FFI
/// . Returns a null pointer in case of error
#[no_mangle]
extern fn mininipNewParser() -> *mut Parser {
    // Since `Box::new` or `Parser::new` may `panic!`, we must use `catch_unwind` because unwinding through FFI is undefined behavior
    catch_unwind(|| {
        ffi_export(Parser::new())
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroys a `Parser` created by `mininipNewParser`
/// . I wrote it to handle error cases but you should implicitly destroy it through `mininipGetParserData` in any normal use case
#[no_mangle]
unsafe extern fn mininipDestroyParser(parser: *mut Parser) {
    // There is no reason for `std::mem::drop` or `Box::from_raw` to `panic!` so I assume it is safe to not `catch_unwind`
    ffi_destroy(parser);
}

/// The data retrieved from a parser
type MininipData = HashMap<Identifier, Value>;

/// Destroys a `Parser` created by `mininipNewParser` and returns the result of `parser.data()` which can be used through FFI
/// . It is useful to retrieve the datas in a parsed file
/// 
/// # Warning
/// The argument `parser` is therefore invalidated and must NOT be used later
#[no_mangle]
unsafe extern fn mininipGetParserData(parser: *mut Parser) -> *mut MininipData {
    // Here, we can `panic!` too
    catch_unwind(|| {
        let parser = Box::from_raw(parser);
        ffi_export(parser.data())
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroys the result of `mininipGetParserData`
#[no_mangle]
unsafe extern fn mininipDestroyParserData(data: *mut MininipData) {
    ffi_destroy(data);
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
    MininipError {
        msg: ffi_export_str(&format!("{}", err)),
        kind: err.into(),
    }
}

/// Destroys an FFI-friendly error
#[no_mangle]
unsafe extern fn mininipDestroyError(err: *mut MininipError) {
    let err = &mut *err;
    if err.msg != std::ptr::null_mut() {
        ffi_destroy_str(err.msg);
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
unsafe extern fn mininipParseFile(path: *const c_char, datas: *mut *mut MininipData) -> MininipError {
    // Extracting a valid path from the argument
    let path = ffi_decode_str(path);
    let path = match path {
        Ok(val) => val,
        Err(_)  => return MininipError {
            msg: ffi_export_str("Argument is not valid utf-8"),
            kind: MininipErrorKind::RuntimeError,
        },
    };

    catch_unwind(|| {
        match parse::parse_file(path) {
            Ok(val) => {
                let ptr = ffi_export(val);
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
                        ptr: ffi_export_str(&s),
                    },
                },
                value_type: MininipType::Raw,
            },
            Value::Str(s) => MininipEntry {
                value: MininipValue {
                    string: MininipStrValue {
                        ptr: ffi_export_str(&s),
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
unsafe extern fn mininipGetEntry(data: *mut MininipData, section: *const c_char, key: *const c_char, entry: *mut MininipEntry) -> MininipBoolValue {
    catch_unwind(|| {
        let section = if section == std::ptr::null() {
            None
        } else {
            match ffi_decode_str(section) {
                Ok(val) => Some(String::from(val)),
                Err(_)  => return MININIP_FALSE,
            }
        };
        let key = match ffi_decode_str(key) {
            Ok(val) => String::from(val),
            Err(_)  => return MININIP_FALSE,
        };

        if let Some(val) = &section {
            if !Identifier::is_valid(val) {
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
        MininipType::Raw   => ffi_destroy_str(entry.value.raw.ptr),
        MininipType::Str   => ffi_destroy_str(entry.value.string.ptr),
        MininipType::Int   => {}, // No ressource to free here
        MininipType::Float => {}, // No ressource to free here
        MininipType::Bool  => {}, // No ressource to free here
        // NOTICE: I could use the `_` pattern here but I wanted an exhaustive match to prevent me from forgetting to update this function when I extend the type
        // system
    }
}

/// An FFI handle to a `Tree`
type MininipTree = crate::datas::tree::Tree;

/// Creates a new `MininipTree` from an existing `MininipData`
/// 
/// # Parameters
/// `data` the data to build a `MininipTree` from. Will be invalidated
/// 
/// # Return value
/// A `MininipTree` holding `data`
/// 
/// A null pointer if any error occurs (always a runtime error such as memory allocation failure)
#[no_mangle]
unsafe extern fn mininipCreateTreeFromData(data: *mut MininipData) -> *mut MininipTree {
    catch_unwind(|| {
        let data = Box::from_raw(data);
        let tree = MininipTree::from(*data);
        ffi_export(tree)
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroys the `MininipTree` passed as parameters
#[no_mangle]
unsafe extern fn mininipDestroyTree(tree: *mut MininipTree) {
    ffi_destroy(tree);
}

/// Releases the `MininipData` used by a `MininipTree`
/// 
/// # Parameters
/// `tree` the `MininipTree` to consume and to extract data from
/// 
/// # Return value
/// A pointer to that `MininipData` or `NULL` if a memory allocation failed
#[no_mangle]
unsafe extern fn mininipGetDataFromTree(tree: *mut MininipTree) -> *mut MininipData {
    catch_unwind(|| {
        let tree = Box::from_raw(tree);
        ffi_export(tree.into_data())
    })
    .unwrap_or(std::ptr::null_mut())
}

/// An iterator over the various sections of a `MininipTree`
// marked as `'static` because the FFI interface is designed to be `'static`. Pointers will live as long as they are not freed
pub struct MininipSectionIterator {
    iterator: crate::datas::tree::SectionIterator<'static>,
    last_allocated: *mut MininipSection,
}

impl Drop for MininipSectionIterator {
    fn drop(&mut self) {
        if self.last_allocated != std::ptr::null_mut() {
            unsafe {
                ffi_destroy(self.last_allocated);
            }
        }
    }
}

/// Returns an iterator over the sections of a `MininipTree`
/// 
/// # Parameters
/// `tree` the tree to iterate on
/// 
/// # Return value
/// A pointer to a new `MininipSectionIterator` over `tree`
/// 
/// # See
/// `mininipDestroySectionIterator` to destroy the returned iterator
#[no_mangle]
unsafe extern fn mininipCreateSectionIterator(tree: *mut MininipTree) -> *mut MininipSectionIterator {
    let tree = &mut *tree;
    let iter = MininipSectionIterator {
        iterator: tree.sections(),
        last_allocated: std::ptr::null_mut(),
    };
    ffi_export(iter)
}

/// Destroys a `MininipSectionIterator`
/// 
/// # Parameters
/// `ptr` a pointer to the `MininipSectionIterator` to destroy
#[no_mangle]
unsafe extern fn mininipDestroySectionIterator(ptr: *mut MininipSectionIterator) {
    ffi_destroy(ptr);
}

/// A handle to a section yielded by a SectionIterator
pub type MininipSection = crate::datas::tree::Section<'static>;

/// Yields the next `MininipSection` from a `MininipSectionIterator` or a null pointer if iteration ended
/// 
/// # Parameters
/// `iter` the `MininipSectionIterator` to yield from
/// 
/// # Return value
/// A pointer to the `MininipSection` yielded from `iter`
/// 
/// # Note
/// You do **not** own the pointer to that `MininipSection` so you do **not** have to free it and you must **not** assume that it will remain valid
/// once you called this function once again
/// 
/// # See
/// `mininipNextOwnedSection` if you want to own the pointer yielded though this is not recommended except when necessary
#[no_mangle]
unsafe extern fn mininipNextSection(iter: *mut MininipSectionIterator) -> *mut MininipSection {
    let iterator = &mut *iter;
    iterator.last_allocated = mininipNextOwnedSection(iter);
    iterator.last_allocated
}

/// Yields the next `MininipSection` from a `MininipSectionIterator` or a null pointer if iteration ended
/// 
/// # Parameters
/// `iter` the `MininipSectionIterator` to yield from
/// 
/// # Return value
/// A pointer to the `MininipSection` yielded from `iter`
/// 
/// # Note
/// You own the pointer to that `MininipSection` so you have to free it and you can assume that it will be kept valid once you called this function
/// once again (except if you free it before)
/// 
/// # See
/// `mininipNextSection` if you do not want to own the pointer yielded (this is the recommended way if owning the pointer is not necessary)
#[no_mangle]
unsafe extern fn mininipNextOwnedSection(iter: *mut MininipSectionIterator) -> *mut MininipSection {
    let iter = &mut *iter;
    if iter.last_allocated != std::ptr::null_mut() {
        mininipDestroySection(iter.last_allocated);
        iter.last_allocated = std::ptr::null_mut();
    }

    match iter.iterator.next() {
        Some(val) => ffi_export(val),
        None      => std::ptr::null_mut(),
    }
}

/// Destroys a `MininipSection`
/// 
/// # Parameters
/// `ptr` the handle to the `MininipSection` to free
#[no_mangle]
unsafe extern fn mininipDestroySection(ptr: *mut MininipSection) {
    ffi_destroy(ptr);
}

/// Returns the name of a `MininipSection`
/// 
/// # Parameters
/// `section` the section to return the name
/// 
/// `ptr` the pointer to assign to the name of `section`. Must be freed using `MininipDestroyString`
/// 
/// # Return value
/// `MININIP_TRUE` in case of success
/// 
/// `MININIP_FALSE` in case of memory allocation error. In this case, `ptr` is not set and must **not** be freed
#[no_mangle]
unsafe extern fn mininipGetSectionName(section: *const MininipSection, ptr: *mut *mut c_char) -> MininipBoolValue {
    let section = &*section;
    match section.name() {
        Some(name) => catch_unwind(|| {
            *ptr = ffi_export_str(name);
            MININIP_TRUE
        })
        .unwrap_or(MININIP_FALSE),
        None => MININIP_TRUE,
    }
}

// unit-tests
#[cfg(test)]
mod tests;
