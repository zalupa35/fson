pub use crate::parser::Rule;
pub use pest::iterators::{Pair, Pairs};
pub use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateValue {
    String(String),
    Interpolation(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReferenceAsValue {
    Id(String),
    Path(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// `inf` in js
    Infinity,
    /// `-inf` in js
    NegativeInfinity,
    /// `null` in js
    Null,
    /// `NaN` in js
    NaN,

    /// ## Example:
    /// ```
    /// Value::String(String::from("string"))
    /// ```
    String(String),

    /// ## Example:
    /// ```
    /// Value::Number(1.0)
    /// ```
    Number(f64),

    /// ## Example:
    /// ```
    /// Value::Boolean(true)
    /// Value::Boolean(false)
    /// ```
    Boolean(bool),

    /// ## Example:
    /// ```
    /// Value::Identifier(String::from("identifier"))
    /// ```
    Identifier(String),

    /// Object: `{ k: v, ... }`
    /// ## Example:
    /// ```
    /// Value::Object(object!(
    ///     String::from("one") => Value::Number(1.0),
    ///     String::from("two") => Value::Number(2.0))
    /// ))
    /// ```
    Object(HashMap<String, Value>),

    /// Array: `[..., ...]`
    /// ## Example:
    /// ```
    /// Value::Array(vec![ Value::Number(1.0), Value::Array(vec![ Value::Boolean(true) ]) ])
    /// ```
    Array(Vec<Value>),

    /// Template string: `${...}`
    /// ## Example:
    /// ```
    /// Value::TemplateString(vec![ TemplateValue::String("2 + 2 = ".to_string()), TemplateValue::Interpolation(Value::Number(4.0)) ]) // 2 + 2 = 4
    /// ```
    TemplateString(Vec<TemplateValue>),

    /// Reference as value: `#id | #"id"` or `#/path/to/object | #"path"/"to"/"object"`
    /// ## Example:
    /// ```
    /// Value::Reference(ReferenceAsValue::Id("id".to_string()))
    /// Value::Reference(ReferenceAsValue::Path(vec![ String::from("path"), String::from("to"), String::from("object") ]))
    /// ```
    Reference(ReferenceAsValue),

    /// Reference declaration:
    /// ```
    /// #{
    ///     #id: "id",
    ///     #value: "any value"
    /// }
    /// ```
    /// ## Example:
    /// ```
    /// Value::ReferenceDeclaration { id: "id".to_string(), value: Box::new("any value".to_string()) }
    /// ```
    ReferenceDeclaration { id: String, value: Box<Value> },
}

#[derive(Debug, Clone)]
pub struct ReferencesManager {
    pub refs: HashMap<String, Value>,
    pub ref_paths: HashMap<String, Value>,
}

impl ReferencesManager {
    pub fn get_by_path(self, path: String) -> Option<Value> {
        if self.ref_paths.contains_key(&path.clone()) {
            let value = &self.ref_paths[&path];
            Some(value.clone())
        } else {
            None
        }
    }
    pub fn get_by_id(self, id: String) -> Option<Value> {
        if self.refs.contains_key(&id.clone()) {
            let value = &self.refs[&id];
            Some(value.clone())
        } else {
            None
        }
    }
}
