use crate::types::*;
use crate::utils::{sanitize_string, stringify_value};

fn get_all_ref_paths(
    pair: Pair<'_, Rule>,
    mut path: String,
    refs_manager: ReferencesManager,
) -> HashMap<String, Pair<'_, Rule>> {
    let mut paths: HashMap<String, Pair<'_, Rule>> = HashMap::new();
    let inner = pair.clone().into_inner();
    let rule = pair.as_rule();

    if rule == Rule::object_pair {
        let mut cloned_inner = inner.clone();
        let index = parse_pair(cloned_inner.next().unwrap(), refs_manager.clone());
        let value = cloned_inner.next().unwrap();

        let mut index_value: String = String::new();
        match index {
            Value::Identifier(str) => index_value = str,
            Value::String(str) => index_value = str,
            _ => {}
        }

        paths.insert(path.clone() + &index_value, value.clone());

        if value.as_rule() == Rule::object {
            path += format!("{}/", index_value).as_str();
        }
    }

    inner.for_each(|e| {
        paths.extend(get_all_ref_paths(e, path.clone(), refs_manager.clone()));
    });
    paths
}

fn check_pair_for_ref(pair: Pair<'_, Rule>) -> HashMap<String, Pair<'_, Rule>> {
    let mut refs: HashMap<String, Pair<'_, Rule>> = HashMap::new();
    let mut inner = pair.clone().into_inner();
    let rule = pair.as_rule();

    if rule == Rule::ref_value {
        let id = sanitize_string(
            inner
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_span()
                .as_str()
                .to_string(),
        );

        let val = inner.next().unwrap().into_inner().next().unwrap();
        refs.insert(id.to_string(), val);
    } else {
        inner.for_each(|e| {
            refs.extend(check_pair_for_ref(e));
        });
    }
    refs
}

pub fn to_hashmap(mut pairs: Pairs<'_, Rule>) -> Result<Value, ()> {
    let first_pair = pairs.next().unwrap();

    let empty_refs_manager = ReferencesManager {
        ref_paths: HashMap::new(),
        refs: HashMap::new(),
    };

    let ref_paths = get_all_ref_paths(
        first_pair.clone(),
        "/".to_string(),
        empty_refs_manager.clone(),
    );
    let refs = check_pair_for_ref(first_pair.clone());

    let mut value_ref_paths: HashMap<String, Value> = HashMap::new();
    let mut value_refs: HashMap<String, Value> = HashMap::new();

    refs.keys().for_each(|key| {
        let value = &refs[key];
        value_refs.insert(
            key.clone(),
            parse_pair(
                value.clone(),
                ReferencesManager {
                    ref_paths: value_ref_paths.clone(),
                    refs: value_refs.clone(),
                },
            ),
        );
    });

    ref_paths.keys().for_each(|key| {
        let value = &ref_paths[key];
        value_ref_paths.insert(
            key.clone(),
            parse_pair(
                value.clone(),
                ReferencesManager {
                    ref_paths: value_ref_paths.clone(),
                    refs: value_refs.clone(),
                },
            ),
        );
    });
    Ok(parse_pair(
        first_pair,
        ReferencesManager {
            ref_paths: value_ref_paths,
            refs: value_refs,
        },
    ))
}

fn parse_pair(pair: Pair<'_, Rule>, refs_manager: ReferencesManager) -> Value {
    match pair.as_rule() {
        Rule::number => parse_number(pair),
        Rule::string => parse_string(pair, refs_manager),
        Rule::not_a_number => Value::NaN,
        Rule::null => Value::Null,
        Rule::identifier => Value::Identifier(pair.as_span().as_str().to_string()),
        Rule::reference => parse_reference(pair, refs_manager),
        Rule::object => parse_object(pair, refs_manager),
        Rule::boolean => Value::Boolean(pair.as_span().as_str() == "true"),
        Rule::ref_value => parse_ref_value(pair, refs_manager),
        Rule::array => parse_array(pair, refs_manager),
        _ => Value::Null,
    }
}

fn parse_ref_value(pair: Pair<'_, Rule>, refs_manager: ReferencesManager) -> Value {
    let mut inner = pair.into_inner();
    inner.next();
    parse_pair(
        inner.next().unwrap().into_inner().next().unwrap(),
        refs_manager,
    )
}

fn parse_array(pair: Pair<'_, Rule>, refs_manager: ReferencesManager) -> Value {
    let mut arr: Vec<Value> = vec![];
    let inner = pair.into_inner();

    inner.for_each(|e| {
        arr.push(parse_pair(e, refs_manager.clone()));
    });

    Value::Array(arr)
}

fn parse_number(pair: Pair<'_, Rule>) -> Value {
    let span = pair.as_span();
    let str = span.as_str();

    match str {
        "Infinity" => Value::Infinity,
        "-Infinity" => Value::NegativeInfinity,
        _ => {
            if let Ok(float) = str.parse::<f64>() {
                Value::Number(float)
            } else if let Ok(hexadecimal) = usize::from_str_radix(str.trim_start_matches("0x"), 16)
            {
                Value::Number(hexadecimal as f64)
            } else {
                Value::Number(0.0)
            }
        }
    }
}

fn parse_template_string(pair: Pair<'_, Rule>, refs_manager: ReferencesManager) -> String {
    let mut result_string = String::new();
    let mut inner = pair.into_inner();
    let template_string = inner.next().unwrap();
    let template_string_inner = template_string.into_inner();

    template_string_inner.for_each(|template_pair| {
        let pair_rule = template_pair.as_rule();
        if pair_rule == Rule::template_char {
            result_string.push_str(template_pair.as_span().as_str());
        } else if pair_rule == Rule::interpolation_template {
            let mut pair_inner = template_pair.into_inner();
            let interpolation_first = pair_inner.next().unwrap();
            let value = parse_pair(interpolation_first.clone(), refs_manager.clone());

            match value {
                Value::String(str) => result_string.push_str(str.as_str()),
                _ => {
                    result_string.push_str(stringify_value(value, 0, false).as_str());
                }
            }
        }
    });

    result_string
}

fn parse_string(pair: Pair<'_, Rule>, refs_manager: ReferencesManager) -> Value {
    let mut inner = pair.clone().into_inner();
    let inner_first = inner.next();

    if let Some(first) = inner_first {
        let first_span = first.as_span();
        let mut str: String = String::new();

        match first.as_rule() {
            Rule::single_quotes_string => {
                str = first_span.as_str().to_string();
            }
            Rule::double_quotes_string => {
                str = first_span.as_str().to_string();
            }
            Rule::template_string => {
                str = parse_template_string(pair, refs_manager);
            }
            _ => {}
        }
        Value::String(sanitize_string(str.to_string()).to_string())
    } else {
        Value::String(String::new())
    }
}

fn parse_object(pair: Pair<'_, Rule>, refs_manager: ReferencesManager) -> Value {
    let mut hashmap: HashMap<String, Value> = HashMap::new();
    let inner = pair.into_inner();

    inner.for_each(|object_pair| {
        let mut pair_inner = object_pair.into_inner();
        let index = parse_pair(pair_inner.next().unwrap(), refs_manager.clone());
        let value = parse_pair(pair_inner.next().unwrap(), refs_manager.clone());

        let mut index_value = String::new();

        match index {
            Value::Identifier(identifier) => index_value = identifier,
            Value::String(str) => index_value = str,
            _ => {}
        }

        hashmap.insert(index_value, value);
    });

    Value::Object(hashmap)
}

fn parse_reference(pair: Pair<'_, Rule>, refs_manager: ReferencesManager) -> Value {
    let mut inner = pair.clone().into_inner();
    let inner_first = inner.next().unwrap();
    let first_rule = inner_first.as_rule();

    if first_rule == Rule::identifier {
        let id = inner_first.as_span().as_str();

        let value = refs_manager.get_by_id(id.to_string());
        value.unwrap_or(Value::Null)
    } else if first_rule == Rule::string {
        let id = parse_pair(inner_first, refs_manager.clone());
        let mut id_str = String::new();

        if let Value::String(i) = id {
            id_str = i
        }

        let value = refs_manager.get_by_id(id_str);
        value.unwrap_or(Value::Null)
    } else {
        let path_inner = inner_first.into_inner();
        let mut path_str = String::new();

        path_inner.for_each(
            |path_pair| match parse_pair(path_pair, refs_manager.clone()) {
                Value::Identifier(identifier) => {
                    path_str += format!("/{}", identifier).as_str();
                }
                Value::String(str) => {
                    path_str += format!("/{}", str).as_str();
                }
                _ => {}
            },
        );

        let value = refs_manager.get_by_path(path_str);
        value.unwrap_or(Value::Null)
    }
}
