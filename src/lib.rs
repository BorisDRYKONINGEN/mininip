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

/// Returns a new `Parser` which can be used through FFI
#[no_mangle]
extern fn mininipNewParser() -> *mut Parser {
    Box::leak(Box::new(Parser::new())) as *mut Parser
}

/// Destroys a `Parser` created by `mininipNewParser`
/// . I wrote it to handle error cases but you should implicitly destroy it through `mininipGetParserData` in any normal use case
#[no_mangle]
unsafe extern fn mininipDestroyParser(parser: *mut Parser) {
    std::mem::drop(Box::from_raw(parser));
}

/// Destroys a `Parser` created by `mininipNewParser` and returns the result of `parser.data()` which can be used through FFI
/// . It is useful to retrieve the datas in a parsed file
/// 
/// # Warning
/// The argument `parser` is therefore invalidated and must NOT be used later
#[no_mangle]
unsafe extern fn mininipGetParserData(parser: *mut Parser) -> *mut HashMap<Identifier, Value> {
    let parser = Box::from_raw(parser);
    Box::leak(Box::new(parser.data())) as *mut HashMap<Identifier, Value>
}

/// Destroys the result of `mininipGetParserData`
#[no_mangle]
unsafe extern fn mininipDestroyParserData(data: *mut HashMap<Identifier, Value>) {
    std::mem::drop(Box::from_raw(data));
}


// unit-tests
#[cfg(test)]
mod tests;
