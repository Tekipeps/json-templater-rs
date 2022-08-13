use std::collections::HashMap;
// use serde_json
use regex::{CaptureMatches, Regex};

pub struct Template2<'a> {
    template: &'a str,
    regex: Regex,
}

impl<'a> Template2<'a> {
    pub fn new(template: &'a str) -> Self {
        let regex = Regex::new(r#"(\{\{([^{}]+)?\}\})"#).unwrap();

        Template2 { template, regex }
    }

    fn get_matches(&self) -> CaptureMatches {
        self.regex.captures_iter(self.template)
    }

    pub fn apply(&self, table: HashMap<String, String>) -> String {
        let captures = self.get_matches();
        let mut template = self.template.clone().to_string();

        let mut i = 0;
        unsafe {
            for cap in captures.into_iter() {
                let name = cap.get(i).map_or("", |m| m.as_str());
                template = template.replace(
                    name,
                    table
                        .get(name.get_unchecked(2..name.len() - 2))
                        .expect("Value not in the lookup table"),
                );
                i += 1;
            }
        }

        return template;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Template2;

    // create test json
    fn get_jayson_() -> &'static str {
        return r#"{
            "index": "myindex",
            "body": {
              "query": {
                "match": {
                  "title": "{{title}}"
                }
              },
              "facets": {
                "tags": {
                  "terms": {
                    "field": "tags",
                    "a": "{{layers}}"
                  }
                }
              }
            }
          }"#;
    }
    #[test]
    fn can_create_new_template() {
        let jayson = get_jayson_();

        let tmplte = Template2::new(jayson);

        assert_eq!(tmplte.template, jayson);
    }

    #[test]
    fn can_parse_demo_json() {
        let jayson = get_jayson_();

        let template = Template2::new(jayson);

        let values = template.get_matches();

        assert_eq!(values.count(), 2);
    }

    #[test]
    fn can_replace_tags_with_values() {
        let jayson = r#"{"match": {
            "title": "{{title}}"
          },
          "terms": {
            "field": "tags",
            "a": "{{layers}}"
          }
        }"#;

        let template = Template2::new(jayson);
        let mut lookup_table = HashMap::new();

        lookup_table.insert("title".to_string(), "Rust in production!".to_string());
        lookup_table.insert("layers".to_string(), "[5, 8, 90]".to_string());

        let jayson = template.apply(lookup_table);

        let output = r#"{"match": {
            "title": "Rust in production!"
          },
          "terms": {
            "field": "tags",
            "a": "[5, 8, 90]"
          }
        }"#;

        assert_eq!(jayson, output)
    }
}
