use crate::stringify_json::stringify;
use crate::Value;

/// Generate FSON from Value
pub fn from(val: Value) -> String {
    stringify(val, 0, false)
}

#[macro_export]
macro_rules! object {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
