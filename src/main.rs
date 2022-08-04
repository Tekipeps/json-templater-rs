use json_templating_rs::template::Template;
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

    let result = Template::new(template);
}
