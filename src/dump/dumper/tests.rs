use crate::dump::dumper::*;
use crate::datas::{Identifier, Value};

#[test]
fn dumper_without_globals() {
    let mut dumper = Dumper::new();

    let abc = Some(String::from("abc"));
    let a = Identifier::new(abc.clone(), String::from("a"));
    let b = Identifier::new(abc.clone(), String::from("b"));
    let c = Identifier::new(abc,         String::from("c"));

    let def = Some(String::from("def"));
    let d = Identifier::new(def.clone(), String::from("d"));
    let e = Identifier::new(def.clone(), String::from("e"));
    let f = Identifier::new(def,         String::from("f"));

    let dump = &mut |ident, val| {
        dumper.dump(ident, Value::Raw(String::from(val)));
    };

    dump(a, "1");
    dump(b, "2");
    dump(c, "3");
    dump(d, "4");
    dump(e, "5");
    dump(f, "6");

    let expected = "\
    [abc]\n\
    a=1\n\
    b=2\n\
    c=3\n\
    \n\
    [def]\n\
    d=4\n\
    e=5\n\
    f=6\n";

    assert_eq!(expected, dumper.generate());
}

#[test]
fn dumper_with_globals() {
    let mut dumper = Dumper::new();

    let a = Identifier::new(None, String::from("a"));
    let b = Identifier::new(None, String::from("b"));
    let c = Identifier::new(None, String::from("c"));

    let def = Some(String::from("def"));
    let d = Identifier::new(def.clone(), String::from("d"));
    let e = Identifier::new(def.clone(), String::from("e"));
    let f = Identifier::new(def,         String::from("f"));

    let dump = &mut |ident, val| {
        dumper.dump(ident, Value::Raw(String::from(val)));
    };

    dump(a, "1");
    dump(b, "2");
    dump(c, "3");
    dump(d, "4");
    dump(e, "5");
    dump(f, "6");

    let expected = "\
    a=1\n\
    b=2\n\
    c=3\n\
    \n\
    [def]\n\
    d=4\n\
    e=5\n\
    f=6\n";

    assert_eq!(expected, dumper.generate());
}

#[test]
fn dumper_with_escape() {
    let mut dumper = Dumper::new();

    let ident = Identifier::new(None, String::from("ident"));
    let val = Value::Raw(String::from(":D = \u{263a}"));

    dumper.dump(ident, val);

    assert_eq!("ident=\\:D \\= \\x00263a\n", dumper.generate());
}
