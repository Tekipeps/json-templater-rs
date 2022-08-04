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
                "cool": "{{co:cool}}",
                "t4":  "{{foo:false}}",
                "t5": "{{Foo:2}}",
                "t6": false,
                "ty": []
              }
            }
          }
        }
      }"#;
    let mut params = HashMap::new();
    params.insert("nice-name", json!(true));
    params.insert("{{nice?@.+=$()*}}", json!("cool"));
    params.insert("Foo", json!([]));
    let tmpl = Template::new(template);

    let k = tmpl.apply(params);
    println!("New Value: {:#?}", k);
}
