#[macro_export]
macro_rules! json_template {
    ($template:expr, $($val:ident),*) => {
       {
        use regex::Regex;
        use serde_json::to_string;
        let regx = Regex::new(r#""\{\{([(\w)\-\+\.,@/\()\?=\*_\$])+(:([^}}])*)?\}\}""#).unwrap();
        let mut t = $template.to_string();

        $(
            if regx.is_match(&format!("\"{{{{{}}}}}\"", stringify!($val))) {
                t = regx
                .replace(
                    t.as_str(),
                    to_string(&$val).unwrap(),
                )
                .to_string()
            }
        )*

        t
       }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_macro() {
        let jayson = r#"{"match": {
            "title": "{{title}}"
          },
          "terms": {
            "field": "tags",
            "a": "{{layers}}"
          }
        }"#;

        let title = "Rust in production!";
        let layers = [5, 8, 90];
        let replaced = json_template!(jayson, title, layers);
        let output = r#"{"match": {
            "title": "Rust in production!"
          },
          "terms": {
            "field": "tags",
            "a": [5,8,90]
          }
        }"#;

        assert_eq!(replaced, output)
    }
}
