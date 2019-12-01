use crate::parse::*;
use crate::datas::{Identifier, Value};

#[test]
fn parser_parse_assignment_simplest() {
    let expr = "ident=val";
    let mut parser = Parser::new();

    parser.parse_assignment(expr)
        .expect("This code should be accepted because it's a valid INI assignment");

    let data = parser.data();
    let key = Identifier::new(None, String::from("ident"));
    let val = Value::Str(String::from("val"));
    assert_eq!(data[&key], val);
}

#[test]
fn parser_parse_assignment_commented() {
    let expr = "ident=val;This is a comment";
    let mut parser = Parser::new();

    parser.parse_assignment(expr)
        .expect("This code should be accepted because it's a valid INI assignment");

    let data = parser.data();
    let key = Identifier::new(None, String::from("ident"));
    let val = Value::Str(String::from("val"));
    assert_eq!(data[&key], val);
}

#[test]
fn parser_parse_assignment_with_spaces() {
    let expr = "ident = val";
    let mut parser = Parser::new();

    parser.parse_assignment(expr)
        .expect("This code should be accepted because it's a valid INI assignment");

    let data = parser.data();
    let key = Identifier::new(None, String::from("ident"));
    let val = Value::Str(String::from("val"));
    assert_eq!(data[&key], val);
}

#[test]
fn parser_parse_assignment_with_comment_and_spaces() {
    let expr = "ident=val ; This is a comment";
    let mut parser = Parser::new();

    parser.parse_assignment(expr)
        .expect("This code should be accepted because it's a valid INI assignment");

    let data = parser.data();
    let key = Identifier::new(None, String::from("ident"));
    let val = Value::Str(String::from("val"));
    assert_eq!(data[&key], val);
}

#[test]
fn parser_parse_assignment_unicode_value() {
    let expr = r"latin_small_letter_e_with_acute=\x0000e9";
    let mut parser = Parser::new();

    parser.parse_assignment(expr)
        .expect("This code should be accepted because it's a valid INI assignment");

    let data = parser.data();
    let key = Identifier::new(None, String::from("latin_small_letter_e_with_acute"));
    let val = Value::Str(String::from("\u{e9}"));
    assert_eq!(data[&key], val);
}

#[test]
fn parser_parse_assignment_unicode_comment() {
    let expr = "ident=val; C'est un cas tout à fait valid"; // Notice the 'à' in the comment
    let mut parser = Parser::new();

    parser.parse_assignment(expr)
        .expect("This code should be accepted because it's a valid INI assignment");

    let data = parser.data();
    let key = Identifier::new(None, String::from("ident"));
    let val = Value::Str(String::from("val"));
    assert_eq!(data[&key], val);
}

#[test]
fn parser_parse_assignment_unicode_identifier() {
    let expr = r"é=\x0000e9";
    let mut parser = Parser::new();

    assert_eq!(parser.parse_assignment(expr), Err(()));
}

#[test]
fn parser_parse_assignment_bad_ident() {
    let expr = "my identifier=val";
    let mut parser = Parser::new();

    assert_eq!(parser.parse_assignment(expr), Err(()));
}

#[test]
fn parser_parse_assignment_bad_value() {
    let expr = "ident=abc=123";
    let mut parser = Parser::new();

    assert_eq!(parser.parse_assignment(expr), Err(()));
}
