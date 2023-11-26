use fson::{parser::parse, stringify_json::stringify, Value};
fn main() {
    // Parsing JSON
    let _parsed = parse(String::from(" 'JSON' ")).unwrap();

    // Stringify JSON
    let _json = stringify(Value::Infinity, 0, false);
}
