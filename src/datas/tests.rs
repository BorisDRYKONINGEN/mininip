use crate::datas::*;

/// Tests only the constant substitutions such as `\` -> `\\` and not the runtime-computed ones
#[test]
fn dump_str_constants_substitutions() {
    assert_eq!(dump_str("\\"),   r"\\");
    assert_eq!(dump_str("'"),    String::from("\\'"));
    assert_eq!(dump_str("\""),   String::from("\\\""));
    assert_eq!(dump_str("\0"),   String::from("\\0"));
    assert_eq!(dump_str("\x07"), String::from("\\a"));
    assert_eq!(dump_str("\x08"), String::from("\\b"));
    assert_eq!(dump_str("\t"),   String::from("\\t"));
    assert_eq!(dump_str("\r"),   String::from("\\r"));
    assert_eq!(dump_str("\n"),   String::from("\\n"));
    assert_eq!(dump_str(";"),    String::from("\\;"));
    assert_eq!(dump_str("#"),    String::from("\\#"));
    assert_eq!(dump_str("="),    String::from("\\="));
    assert_eq!(dump_str(":"),    String::from("\\:"));
}

#[test]
fn dump_str_dynamic_substitutions() {
    assert_eq!(dump_str("\u{00263a}"), String::from("\\x00263a"));
    assert_eq!(dump_str("\u{000100}"), String::from("\\x000100"));
    assert_eq!(dump_str("\u{01342e}"), String::from("\\x01342e"));
}

#[test]
fn dump_str_ignore() {
    assert_eq!(dump_str("abc123"), String::from("abc123"));
}

#[test]
fn dump_str_complementary_test() {
    assert_eq!(dump_str("très_content=☺ ; the symbol of hapiness"), "tr\\x0000e8s_content\\=\\x00263a \\; the symbol of hapiness");
}

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
