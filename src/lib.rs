use serde_json::{Result, Value};

pub fn meh() -> Value {
    let data = r#"
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

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();

    return v;
}

#[cfg(test)]
mod tests {
    use crate::meh;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_meh() {
        let v = meh();
        // Access parts of the data by indexing with square brackets.
        println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    }
}
