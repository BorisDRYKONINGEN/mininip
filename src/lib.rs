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

/// Returns a new `Parser` which can be used through FFI
/// . Returns a null pointer in case of error
#[no_mangle]
extern fn mininipNewParser() -> *mut Parser {
    // Since `Box::new` or `Parser::new` may `panic!`, we must use `catch_unwind` because unwinding through FFI is undefined behavior
    catch_unwind(|| {
        Box::leak(Box::new(Parser::new())) as *mut Parser
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
        Box::leak(Box::new(parser.data())) as *mut HashMap<Identifier, Value>
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroys the result of `mininipGetParserData`
#[no_mangle]
unsafe extern fn mininipDestroyParserData(data: *mut HashMap<Identifier, Value>) {
    std::mem::drop(Box::from_raw(data));
}


// unit-tests
#[cfg(test)]
mod tests;
