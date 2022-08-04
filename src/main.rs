use serde_json::{Deserializer, Serializer, Value};

use regex::{bytes::CaptureMatches, Regex};

fn parse(params: Vec<String>, template: &str) -> String {
    let reg: Regex = Regex::new(r#"\{\{([A-Za-z0-9-+.,@/\()?=*_$])+(:(.)+)*\}\}"#).unwrap();
    let template_json: Value = serde_json::from_str(template).expect("Expect a valid Json");

    let y = reg.captures_iter(template);

    for a in y {
        println!("{:#?}", a);
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
              "title": "{{Foo: true}}"
            }
          },
          "facets": {
            "tags": {
              "terms": {
                "field": "tags",
                "cool": "{{cool:asd}}",
                "tt": "{{:}}"
              }
            }
          }
        }
      }"#;

    let result = parse(Vec::new(), template);

    let y = r##"{ "key": " { "key2": "val" }" }"##;
    let k = r#"{"sdf": "["a"]"}"#;
    // let _b: Value = serde_json::from_str(k).expect("Failed to parse");
    // println!("{}", result);
    println!("{}", k.to_string())
}
