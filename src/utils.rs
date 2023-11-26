use super::{
    stringify_json,
    types::{ReferenceAsValue, Value},
    TemplateValue,
};

pub fn sanitize_string(str: String) -> String {
    str.replace('\"', "\\\"")
        .replace("\\'", "'")
        .replace("\\`", "`")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\n")
}

pub fn stringify_value(value: Value, ident: usize, minify: bool) -> String {
    match value {
        Value::Number(num) => num.to_string(),
        Value::Boolean(bool) => bool.to_string(),
        Value::Infinity => String::from("Infinity"),
        Value::NegativeInfinity => String::from("-Infinity"),
        Value::Null => String::from("null"),
        Value::NaN => String::from("NaN"),
        Value::String(str) => format!("\"{}\"", sanitize_string(str)),

        Value::Array(arr) => {
            let mut str = String::from("[");
            for (i, e) in arr.iter().enumerate() {
                str.push_str(
                    format!(
                        "{}{}",
                        stringify_value(e.clone(), ident, minify),
                        if i == arr.len() - 1 {
                            ""
                        } else if minify {
                            ","
                        } else {
                            ", "
                        }
                    )
                    .as_str(),
                );
            }
            str + "]"
        }

        Value::Object(_) => stringify_json::stringify(value, ident, minify),

        Value::TemplateString(val) => {
            let mut str = String::from("`");
            for e in val {
                match e {
                    TemplateValue::String(s) => str += &s,
                    TemplateValue::Interpolation(interpolation) => {
                        str += format!("${{{}}}", stringify_value(interpolation, ident, minify))
                            .as_str();
                    }
                }
            }
            str + "`"
        }

        Value::ReferenceDeclaration { id, value } => {
            let whitespace = if minify { "" } else { " " };
            format!(
                "#{{{}#id:{}{};{}#value:{}{};{}}}",
                whitespace,
                whitespace,
                stringify_value(Value::String(id), ident, minify),
                whitespace,
                whitespace,
                stringify_value(*value, ident, minify),
                whitespace,
            )
        }
        Value::Reference(reference_value) => match reference_value {
            ReferenceAsValue::Id(id) => {
                format!("#\"{}\"", sanitize_string(id))
            }
            ReferenceAsValue::Path(path) => {
                let mut sanitized_path: Vec<String> = vec![];
                for e in path {
                    sanitized_path.push(format!("\"{}\"", sanitize_string(e)));
                }
                format!("#/{}", sanitized_path.join("/"))
            }
        },

        _ => String::new(),
    }
}
