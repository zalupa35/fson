use fson::{generator, object, parser::parse, ReferenceAsValue, TemplateValue, Value};
fn main() {
    // Generate FSON
    let fson = generator::from(Value::Object(object!(
        String::from("server") => Value::Object(object!(
            String::from("host") => Value::ReferenceDeclaration {id: String::from("host"), value: Box::new(Value::String(String::from("localhost")))},
            String::from("protocol") => Value::ReferenceDeclaration {id: String::from("protocol"), value: Box::new(Value::String(String::from("http")))},
            String::from("port") => Value::ReferenceDeclaration {id: String::from("port"), value: Box::new(Value::Number(80.0))}
        )),
        String::from("indexRoute") => Value::String(String::from("/")),
        String::from("address") => Value::TemplateString(vec![
            TemplateValue::Interpolation(Value::Reference(ReferenceAsValue::Id(String::from("protocol")))),
            TemplateValue::String(String::from("://")),
            TemplateValue::Interpolation(Value::Reference(ReferenceAsValue::Id(String::from("host")))),
            TemplateValue::String(String::from(":")),
            TemplateValue::Interpolation(Value::Reference(ReferenceAsValue::Id(String::from("port")))),
            TemplateValue::Interpolation(Value::Reference(ReferenceAsValue::Path(vec![
                String::from("indexRoute")
            ])))
        ])
    )));

    // Render it
    let rendered = generator::from(parse(fson).unwrap());
    println!("{rendered}");
}
