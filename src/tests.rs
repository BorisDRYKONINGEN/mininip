use crate::{parse, dump, errors};
use parse::parse_file;
use errors::ParseFileError;
use crate::datas::{Identifier, Value};

#[test]
fn parse_reverses_dump() {
    let message = "Hello world ☺. 1+1=2; 2+2=4 \\0/";

    assert_eq!(parse::parse_str(&dump::dump_str(message)).expect("`dump_str` must return a well escaped string"), message);
}

#[test]
fn parse_good_file() {
    let data = parse_file("good.ini").unwrap();

    let author = Identifier::new(None, String::from("author"));
    let version_major = Identifier::new(None, String::from("version_major"));

    let numbers = Some(String::from("numbers"));
    let one = Identifier::new(numbers.clone(), String::from("one"));
    let two = Identifier::new(numbers.clone(), String::from("two"));
    let three = Identifier::new(numbers, String::from("three"));

    let symbols = Some(String::from("symbols"));
    let smiley = Identifier::new(symbols.clone(), String::from("smiley"));
    let semicolon = Identifier::new(symbols, String::from("semicolon"));

    println!("{:?}", data);

    assert_eq!(data[&author], Value::Raw(String::from("Boris DRYKONINGEN")));
    assert_eq!(data[&version_major], Value::Raw(String::from("0")));

    assert_eq!(data[&one], Value::Raw(String::from("1")));
    assert_eq!(data[&two], Value::Raw(String::from("2")));
    assert_eq!(data[&three], Value::Raw(String::from("3")));

    assert_eq!(data[&smiley], Value::Raw(String::from("\u{263a}")));
    assert_eq!(data[&semicolon], Value::Raw(String::from(";")));
}

#[test]
fn parse_bad_file() {
    let err = parse_file("bad.ini");
    match err {
        Ok(_)                              => panic!("This file contains wrong code and shouldn't be allowed"),
        Err(ParseFileError::ParseError(_)) => {},
        Err(err)                           => panic!("Wrong error value returned: {:?}", err),
    }
}

#[test]
fn parse_non_existing_file() {
    let err = parse_file("This file shouldn't exist. If you see it, remove it now.ini");
    match err {
        Ok(_)                           => panic!("This file does not exist. If it exists, remove it"),
        Err(ParseFileError::IOError(_)) => {},
        Err(err)                        => panic!("Wrong error value returned: {:?}", err),
    }
}
