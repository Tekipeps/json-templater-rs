use std::collections::HashMap;

use json_templating_rs::template::Template;
use serde_json::{json, Value};
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
                "cool": "{{co:oooo:o}}",
                "tt": "{{nice?@.+=$()*: 2}}",
                "t4":  "{{foo:false}}",
                "t5": "{{Foo}}",
                "t5": "{{nice-name: false}}",
                "t6": "{{nice_name: false}}"

              }
            }
          }
        }
      }"#;
    let mut params = HashMap::new();
    params.insert("nice-name", json!("true"));
    params.insert("{{nice?@.+=$()*}}", json!("cool"));
    params.insert("Foo", json!("[]"));
    let tmpl = Template::new(template);

    println!(
        "error: {:#?}",
        serde_json::from_str::<Value>(r#""sdf:sdf""#)
    );
    // tmpl.apply(params);
}
