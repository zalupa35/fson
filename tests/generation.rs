use fson::{generator, Value};

#[test]
fn create() {
    assert_eq!(generator::from(Value::Null), String::from("null"));
}
