//28. Implement a `JsonValue` enum that can represent JSON data types.

use std::collections::HashMap;

#[derive(Debug)]

enum JsonValue{
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue{
    fn print_preety(&self, indent: usize){
        let space = " ".repeat(indent);
        match self{
            JsonValue::String(s) => println!("{}\"{}\"", space, s),
            JsonValue::Number(n) => println!("{}{}", space,n),
            JsonValue::Boolean(b) => println!("{}{}", space, b),
            JsonValue::Null => println!("{}null", space),
            JsonValue::Array(arr) => {
                println!("{}[", space);
                for value in arr {
                    value.print_preety(indent + 2);
                }
                println!("{}]", space);
            }
            JsonValue::Object(obj) => {
                println!("{}{{", space);
                for (key, value) in obj {
                    print!("{}\"{}\": ", space, key);
                    value.print_preety(indent + 2);
                }
                println!("{}{}}", space);
            }
        }
    }
}

fn main() {
    let json = JsonValue::Object(HashMap::from([
        ("name".to_string(), JsonValue::String("Alice".to_string())),
        ("age".to_string(), JsonValue::Number(30.0)),
        ("is_student".to_string(), JsonValue::Boolean(false)),
        ("courses".to_string(), JsonValue::Array(vec![
            JsonValue::String("Math".to_string()),
            JsonValue::String("Science".to_string()),
        ])),
        ("address".to_string(), JsonValue::Object(HashMap::from([
            ("city".to_string(), JsonValue::String("Wonderland".to_string())),
            ("zip".to_string(), JsonValue::Null),
        ]))),
    ]));

    json.print_pretty(0);
}