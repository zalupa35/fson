use super::types::*;
use super::utils::*;

const ADD_TO_IDENT: usize = 2;
const NEWLINE: &str = "\n";

/// Stringify value
/// ## Example:
/// ```
/// stringify(Value::Number(1.0), 0, false)
/// ```
pub fn stringify(val: Value, mut ident: usize, minify: bool) -> String {
    match val {
        Value::Object(object) => {
            let mut str = String::new();
            str += "{".to_string().as_str();
            let mut add_to_ident = ADD_TO_IDENT;
            let mut newline = NEWLINE;
            if minify {
                ident = 0;
                add_to_ident = 0;
                newline = "";
            };
            let newline_with_ident = String::from(newline) + &" ".repeat(ident);
            for pair in object.clone() {
                let (k, v) = pair;
                str += format!(
                    "{}{}\"{}\":{}{},",
                    if object.is_empty() { "" } else { newline },
                    " ".repeat(ident + add_to_ident),
                    sanitize_string(k),
                    if minify { "" } else { " " },
                    stringify(v, ident + add_to_ident, minify)
                )
                .as_str();
            }
            str += format!(
                "{}{}",
                if object.is_empty() {
                    ""
                } else {
                    newline_with_ident.as_str()
                },
                "}",
            )
            .as_str();
            str
        }
        _ => {
            stringify_value(val, ident, minify)
        }
    }
}
