use crate::parse::*;

#[test]
fn token_iterator_no_escapes() {
    let message = "Hello world!";
    let found = TokenIterator::from(message.chars())
                .collect::<Vec<Token>>();

    let expected = message.chars()
                          .map(|c| Token::Char(c))
                          .collect::<Vec<Token>>();

    assert_eq!(found, expected);
}

#[test]
fn token_iterator_special_escapes() {
    let message = "\\;\\:\\=\\\\\\\'\\\"";
    let found = TokenIterator::from(message.chars())
                .collect::<Vec<Token>>();

    let expected = [r"\;", r"\:", r"\=", r"\\", r"\'", "\\\""]
                   .iter()
                   .map(|s| Token::Escape(String::from(*s)))
                   .collect::<Vec<Token>>();

    assert_eq!(found, expected);
}

#[test]
fn token_iterator_unicode_escapes() {
    let message = r"\x00263a\x002665\x000100";
    let found = TokenIterator::from(message.chars())
                .collect::<Vec<Token>>();

    let expected = [r"\x00263a", r"\x002665", r"\x000100"]
                   .iter()
                   .map(|s| Token::Escape(String::from(*s)))
                   .collect::<Vec<Token>>();

    assert_eq!(found, expected);
}

#[test]
fn token_iterator_unfinished_escape() {
    let message = r"Hello\";
    let found = TokenIterator::from(message.chars())
                .collect::<Vec<Token>>();

    let mut expected = message.chars()
                              .take(message.len() - 1)
                              .map(|c| Token::Char(c))
                              .collect::<Vec<Token>>();
    expected.push(Token::Escape(String::from("\\")));

    assert_eq!(found, expected);
}

#[test]
fn parse_str_ignore() -> Result<(), ()> {
    let message = "Hello world";

    assert_eq!(message, parse_str(message)?);
    Ok(())
}

#[test]
fn parse_str_special_escapes() -> Result<(), ()> {
    let message = "\\a\\b\\;\\:\\=\\'\\\"\\t\\r\\n\\0\\\\";
    let expected = "\x07\x08;:='\"\t\r\n\0\\";

    assert_eq!(parse_str(message)?, expected);
    Ok(())
}

#[test]
fn parse_str_unicode_escapes() -> Result<(), ()> {
    let message = r"\x00263a\x002665\x000100";
    let expected = "\u{263a}\u{2665}\u{100}";

    assert_eq!(parse_str(message)?, expected);
    Ok(())
}

#[test]
fn parse_str_unfinished_escape() {
    let message = r"Hello\";

    assert_eq!(parse_str(message), Err(()));
}

#[test]
fn parse_str_forbidden_ascii() {
    let message = r"hello=world";

    assert_eq!(parse_str(message), Err(()));
}

#[test]
fn parse_str_forbidden_unicode() {
    let message = "☺";

    assert_eq!(parse_str(message), Err(()));
}
