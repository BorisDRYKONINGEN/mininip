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
