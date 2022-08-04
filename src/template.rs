use crate::matches::Match;
use regex::{Error, Regex};
use serde_json::{de, json, Result, Value};
use std::collections::{hash_map, HashMap};

pub struct Template {
    template_str: String,
    regex_pattern: Regex,
}

impl Template {
    pub fn new(s: &str) -> Self {
        let y: Template = Template {
            template_str: s.to_string(),
            regex_pattern: Regex::new(r#"\{\{([(\w)\-\+\.,@/\()\?=\*_\$])+(:(.)*)?\}\}"#).unwrap(),
        };
        y
    }
    fn parse(&self) -> Vec<Match> {
        //validate json
        serde_json::from_str::<Value>(&self.template_str).unwrap();

        let matches = self
            .regex_pattern
            .captures_iter(&self.template_str)
            .map(|a| {
                let b = a.get(0).unwrap().as_str();
                Match::convert_string_to_match(b.to_string())
            })
            .collect();
        matches
    }
    pub fn apply(&self, params: HashMap<&str, Value>) -> Value {
        let y = json!("cc");
        let mut replaced_string = self.template_str.clone();

        let matches = self
            .regex_pattern
            .captures_iter(&mut replaced_string.clone())
            .for_each(|found| {
                let found = found
                    .get(0)
                    .expect("Unable to extract from template ")
                    .as_str();
                let found = Match::convert_string_to_match(found.to_string());
                let key = found.get_key();

                let mut opt_actual_value = params.get(key).cloned();
                let actual_value = opt_actual_value.unwrap_or_else(|| {
                    if let Some(default_value) = found.get_default_value() {
                        return serde_json::from_str::<Value>(default_value.as_str()).expect(
                            format!("Invalid JSON for the default value of {}", key).as_str(),
                        );
                    }
                    panic!(
                        "{} has no default value. Add a value in params or a default value!",
                        key
                    );
                });
                //check default and non existing values

                // TODO: escape key names before regex check
                let mut key_reg =
                    Regex::new(format!(r#"\{{\{{{}(:)?\}}\}}"#, key).as_str()).unwrap();
                if let Some(y) = found.get_default_value() {
                    key_reg =
                        Regex::new(format!(r#"\{{\{{{}(:{})?\}}\}}"#, key, y).as_str()).unwrap();
                }

                // println!("{} {:#?}", key_reg, key_reg.find(replaced_string.as_str()));
                replaced_string = key_reg
                    .replace(replaced_string.as_str(), actual_value.to_string())
                    .to_string();

                println!("{}", replaced_string);
            });

        y
    }
}
