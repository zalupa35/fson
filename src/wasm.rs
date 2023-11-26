use std::collections::HashMap;

use crate::{parser, stringify_json, ReferenceAsValue, TemplateValue, Value};
use js_sys::{Array, Number, Object, Reflect};
use wasm_bindgen::prelude::*;

fn value_to_jsvalue(value: Value, ident: usize, minify: bool) -> JsValue {
    match value {
        Value::String(str) => JsValue::from(str.as_str()),
        Value::NaN => JsValue::from(Number::NAN),
        Value::Infinity => JsValue::from(Number::POSITIVE_INFINITY),
        Value::NegativeInfinity => JsValue::from(Number::NEGATIVE_INFINITY),
        Value::Null => JsValue::null(),
        Value::Boolean(b) => JsValue::from_bool(b),
        Value::Number(n) => JsValue::from_f64(n),
        Value::Object(hashmap) => {
            let obj = Object::new();
            for (k, v) in hashmap {
                Reflect::set(
                    &obj,
                    &JsValue::from(k.as_str()),
                    &value_to_jsvalue(v, ident, minify),
                )
                .unwrap();
            }
            JsValue::from(obj)
        }
        _ => JsValue::null(),
    }
}

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

fn jsvalue_to_value(value: JsValue) -> Result<Value, String> {
    //alert("hi");
    if value.as_string().is_some() {
        Ok(Value::String(value.as_string().unwrap()))
    } else if value.as_f64().is_some() {
        let str = value.as_f64().unwrap().to_string();
        if str == "inf" {
            Ok(Value::Infinity)
        } else if str == "-inf" {
            Ok(Value::NegativeInfinity)
        } else if str == "NaN" {
            Ok(Value::NaN)
        } else {
            Ok(Value::Number(value.as_f64().unwrap()))
        }
    } else if value.as_bool().is_some() {
        Ok(Value::Boolean(value.as_bool().unwrap()))
    } else if value.is_array() {
        let arr: Array = value.dyn_into().unwrap();
        let mut vec: Vec<Value> = vec![];
        arr.for_each(&mut |v, i, _| {
            vec.insert(i as usize, jsvalue_to_value(v).unwrap());
        });
        Ok(Value::Array(vec))
    } else if value.is_object() {
        let obj: Object = value.dyn_into().unwrap();
        let mut hashmap: HashMap<String, Value> = HashMap::new();
        for key in Object::keys(&obj) {
            if key.as_string().is_some() {
                let val = Reflect::get(&obj, &key);
                if val.is_ok() {
                    hashmap.insert(
                        key.as_string().unwrap(),
                        jsvalue_to_value(val.unwrap()).unwrap(),
                    );
                }
            } else {
                return Err("Key must be string".to_string());
            }
        }

        if hashmap.len() == 2 && hashmap.contains_key("#id") && hashmap.contains_key("#value") {
            let id_val = hashmap.get("#id").unwrap();
            let mut _id_str = String::new();
            match id_val {
                Value::String(s) => {
                    _id_str = s.to_string();
                }
                _ => {
                    return Err("Id must be string".to_string());
                }
            }
            return Ok(Value::ReferenceDeclaration {
                id: _id_str,
                value: Box::new(hashmap.get("#value").unwrap().clone()),
            });
        } else if hashmap.len() == 1 && hashmap.contains_key("#reference_id") {
            let id_val = hashmap.get("#reference_id").unwrap();
            let mut _id_str = String::new();
            match id_val {
                Value::String(s) => {
                    _id_str = s.to_string();
                }
                _ => {
                    return Err("Id must be string".to_string());
                }
            }
            return Ok(Value::Reference(ReferenceAsValue::Id(_id_str)));
        } else if hashmap.len() == 1 && hashmap.contains_key("#reference_path") {
            let path_val = hashmap.get("#reference_path").unwrap();
            let mut path_arr: Vec<String> = Vec::new();
            match path_val {
                Value::Array(arr) => {
                    for e in arr {
                        match e {
                            Value::String(s) => path_arr.push(s.to_string()),
                            _ => {}
                        }
                    }
                }
                _ => {
                    return Err("Path must be array".to_string());
                }
            }
            return Ok(Value::Reference(ReferenceAsValue::Path(path_arr)));
        } else if hashmap.len() == 1 && hashmap.contains_key("@template_string") {
            let template_value = hashmap.get("@template_string").unwrap();
            let mut template_vec: Vec<TemplateValue> = vec![];
            match template_value {
                Value::Array(arr) => {
                    for e in arr {
                        match e {
                            Value::String(str) => {
                                template_vec.push(TemplateValue::String(str.to_string()))
                            }
                            _ => template_vec.push(TemplateValue::Interpolation(e.clone())),
                        }
                    }
                }
                _ => {
                    return Err("Interpolation value must be array".to_string());
                }
            };
            return Ok(Value::TemplateString(template_vec));
        }

        Ok(Value::Object(hashmap))
    } else {
        Ok(Value::Null)
    }
}

fn parse_from_string(str: String, ident: usize, minify: bool) -> Result<JsValue, String> {
    match parser::parse(str) {
        Ok(v) => Ok(value_to_jsvalue(v, ident, minify)),
        Err(e) => Err(e),
    }
}

#[wasm_bindgen]
pub fn stringify(val: JsValue, ident: usize, minify: bool) -> Result<String, JsError> {
    match jsvalue_to_value(val) {
        Ok(v) => Ok(stringify_json::stringify(v, ident, minify)),
        Err(e) => Err(JsError::new(e.as_str())),
    }
}

#[wasm_bindgen]
pub fn parse(val: String) -> Result<JsValue, JsError> {
    match parse_from_string(val, 0, false) {
        Ok(v) => Ok(v),
        Err(e) => Err(JsError::new(e.as_str())),
    }
}
