use crate::datas::*;

#[test]
fn value_from_string() -> Result<(), String> {
    let txt = String::from("Hello world!");
    let val = Value::from(txt.clone());

    match val {
        Value::Str(string) => if &string == &txt {
                                  Ok(())
                              } else {
                                  Err(string)
                              },
        // Uncoment the line below as soon as two differents types are supported by `Value` which aims to do
        //_                  => Err(format!("{:?}", val)),
    }
}

#[test]
fn value_display() {
    let txt = "Hello world!";
    let val = Value::from(String::from(txt));

    assert_eq!(format!("{}", val), txt);
}

#[test]
fn value_dump() {
    let val = Value::from(String::from("très_content=☺ ; the symbol of hapiness"));
    let dumped = val.dump();

    assert_eq!(dumped, "'tr\\x0000e8s_content\\=\\x00263a \\; the symbol of hapiness'");
}

#[test]
fn value_parse_ok() -> Result<(), ()> {
    let val = Value::parse_str(r"Hello \x002665").unwrap();

    assert_eq!(val, Value::Str(String::from("Hello \u{2665}")));
    Ok(())
}

#[test]
fn value_parse_err() {
    let val = Value::parse_str(r"Hello \p");

    assert!(val.is_err());
}

#[test]
fn identifier_new_some() {
    let section = Some(String::from("Section_name"));
    let variable = String::from("Variable_name");
    let ident = Identifier::new(section.clone(), variable.clone());

    assert_eq!(ident, Identifier { section, name: variable });
}

#[test]
fn identifier_new_none() {
    let section = None;
    let variable = String::from("Variable_name");
    let ident = Identifier::new(section.clone(), variable.clone());

    assert_eq!(ident, Identifier { section, name: variable });
}

#[test]
#[should_panic]
fn identifier_new_panics() {
    let section = Some(String::from("Hello world"));
    let variable = String::from("regular_name");
    let _ident = Identifier::new(section, variable);
}

#[test]
fn identifier_is_valid_full_test() {
    assert!(Identifier::is_valid("UPPERCASE_ONE"));
    assert!(Identifier::is_valid("lowercase_one"));
    assert!(Identifier::is_valid("alpha_numeric_42"));
    assert!(!Identifier::is_valid("42_starts_with_a_digit"));
    assert!(!Identifier::is_valid("Non numeric nor alphabetic character"));
    assert!(!Identifier::is_valid("Non_ascii_character_\u{263a}"));
    assert!(!Identifier::is_valid(""));
}

#[test]
fn identifier_change_section_ok() {
    let mut ident = Identifier::new(Some(String::from("Section")), String::from("Variable"));

    ident.change_section(Some(String::from("Valid_one")));
}

#[test]
#[should_panic]
fn identifier_change_section_err() {
    let mut ident = Identifier::new(Some(String::from("Section")), String::from("Variable"));

    ident.change_section(Some(String::from("Invalid one")));
}

#[test]
fn identifier_change_section_none() {
    let mut ident = Identifier::new(Some(String::from("Section")), String::from("Variable"));

    ident.change_section(None);
}

#[test]
fn identifier_change_name_ok() {
    let mut ident = Identifier::new(Some(String::from("Section")), String::from("Variable"));

    ident.change_name(String::from("Valid_one"));
}

#[test]
#[should_panic]
fn identifier_change_name_err() {
    let mut ident = Identifier::new(Some(String::from("Section")), String::from("Variable"));

    ident.change_name(String::from("Invalid one"));
}

#[test]
fn identifier_format_with_section() {
    let section = String::from("Section");
    let variable = String::from("Variable");
    let ident = Identifier::new(Some(section.clone()), variable.clone());

    assert_eq!(format!("{}", ident), format!("{}.{}", section, variable));
}

#[test]
fn identifier_format_without_section() {
    let section = None;
    let variable = String::from("Variable");
    let ident = Identifier::new(section, variable.clone());

    assert_eq!(format!("{}", ident), variable);
}
