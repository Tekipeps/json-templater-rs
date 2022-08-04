use serde_json::{Deserializer, Serializer, Value};

use regex::{bytes::CaptureMatches, Regex};

#[derive(Clone, Debug, Default)]
struct Match {
    key: String,
    default_value: Option<String>,
}

fn parse(params: Vec<String>, template: &str) -> Vec<Match> {
    let reg: Regex = Regex::new(r#"\{\{([(\w)\-\+\.,@/\()\?=\*_\$])+(:(.)*)?\}\}"#).unwrap();

    //validate json
    serde_json::from_str::<Value>(template).expect("Expect a valid Json");

    let matches = reg
        .captures_iter(template)
        .map(|a| {
            let b = a.get(0).unwrap().as_str();
            convert_string_to_match(b.to_string())
        })
        .collect();

    println!("{:#?}", matches);
    matches
}

fn convert_string_to_match(str: String) -> Match {
    let mut val = Match::default();

    // remove {{ and }}
    let mut str = str.replace("{{", "");
    str = str.replace("}}", "");

    match str.split_once(":") {
        Some((a, b)) => {
            val.key = a.to_string();
            val.default_value = Some(b.to_string());
        }
        None => {
            val.key = str;
        }
    }
    val
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
                "cool": "{{co:oooo:o}}",
                "tt": "{{nice?@.+=$()*}}",
                "tt2": "{{nice-name}}",
                "tt3": "{{nice_name}}",
                "tt3": "{{nice+name}}"

              }
            }
          }
        }
      }"#;

    let result = parse(Vec::new(), template);

    let k = r#" 1"#;
    let b: Value = serde_json::from_str(k).expect("Failed to parse");
    // println!("{}", result);
    println!("{}", b)
}
