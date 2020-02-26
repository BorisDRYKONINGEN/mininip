use crate::datas::{tree::*, Identifier, Value};

#[test]
fn cache_from_data() {
    let mut data = HashMap::new();

    let section = None;
    data.insert(Identifier::new(section.clone(), String::from("version")), Value::Str(String::from("1.3.0")));
    data.insert(Identifier::new(section.clone(), String::from("debug")), Value::Bool(true));
    data.insert(Identifier::new(section,         String::from("allow-errors")), Value::Bool(false));

    let section = Some(String::from("foo"));
    data.insert(Identifier::new(section.clone(), String::from("answer")), Value::Int(42));
    data.insert(Identifier::new(section,         String::from("pi")), Value::Float(3.14));

    let section = Some(String::from("bar"));
    data.insert(Identifier::new(section.clone(), String::from("baz")), Value::Raw(String::new()));
    data.insert(Identifier::new(section,         String::from("abc")), Value::Str(String::from("def")));

    let cache = Cache::from(&data);
    assert_eq!(&cache.sections, &vec![String::from("bar"), String::from("foo")]);

    let global = &cache.keys[&None];
    assert_eq!(global, &vec![String::from("allow-errors"), String::from("debug"), String::from("version")]);

    let foo = &cache.keys[&Some(String::from("foo"))];
    assert_eq!(foo, &vec![String::from("answer"), String::from("pi")]);

    let bar = &cache.keys[&Some(String::from("bar"))];
    assert_eq!(bar, &vec![String::from("abc"), String::from("baz")]);
}
