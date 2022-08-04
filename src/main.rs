use std::collections::HashMap;

use json_templating_rs::template::Template;
use serde_json::json;
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
                "t4":  "{{foo:false}}"
              }
            }
          }
        }
      }"#;
    let mut params = HashMap::new();
    params.insert("nice-name", json!("true"));
    params.insert("{{nice?@.+=$()*}}", json!("cool"));
    let tmpl = Template::new(template);
    tmpl.apply(params);
}
