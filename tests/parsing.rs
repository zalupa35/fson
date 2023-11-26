use fson::{object, parser::parse, Value};

#[test]
fn basic() {
    assert_eq!(
        parse(String::from(
            "{
      x: { y: { z: 'hello' } },
      string: `${#/x/y/z} world`
    }"
        ))
        .unwrap(),
        Value::Object(object!(
          String::from("x") => Value::Object(object!(String::from("y") => Value::Object(object!(String::from("z") => Value::String(String::from("hello")))))),
          String::from("string") => Value::String(String::from("hello world"))
        ))
    );
}
