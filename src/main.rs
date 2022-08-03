use serde_json::{Deserializer, Serializer, Value};

use regex::Regex;

fn parse(params: Vec<String>, template: &str) -> String {
    let reg: Regex = Regex::new(r"\{\{(\w|:|[\s\-+.,@/\\()?=*_$])+}}").unwrap();
    let template: Value = serde_json::from_str(template).unwrap();

    match reg.find(template.to_string().as_str()) {
        Some(x) => println!("{:?}", x),
        None => println!("{}", "None"),
    }

    template.to_string()
}
fn main() {
    let template = r#"
    {
        "index": "myindex",
        "body": {
          "query": {
            "match": {
              "title": "{{title:test}}"
            }
          },
          "facets": {
            "tags": {
              "terms": {
                "field": "tags"
              }
            }
          }
        }
      }"#;

    let result = parse(Vec::new(), template);

    println!("{}", result);
}
