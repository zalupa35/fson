use crate::Value;
use pest::Parser;
mod parse;

#[derive(Parser)]
#[grammar = "parser/grammar/json.pest"]
struct JsonParser;

/// Parses string
/// ## Example:
/// ```
/// parse(r#"
/// {
///     x: 1,
///     y: "string"
/// }
/// "#)
/// ```
pub fn parse(json: String) -> Result<Value, String> {
    match JsonParser::parse(Rule::document, json.as_str()) {
        Ok(pairs) => match parse::to_hashmap(pairs) {
            Ok(result) => Ok(result),
            Err(_) => Err(String::new()),
        },
        Err(e) => Err(e.to_string()),
    }
}
