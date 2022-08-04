use crate::matches::Match;
use regex::{Error, Regex};
use serde_json::{de, json, Result, Value};
use std::collections::{hash_map, HashMap};

pub struct Template {
    template_str: String,
    regex_pattern: Regex,
}

// \{\{([(\w)\-\+\.,@/\()\?=\*_\$])+(:(.)*)?\}\}
impl Template {
    pub fn new(s: &str) -> Self {
        let y: Template = Template {
            template_str: s.to_string(),
            regex_pattern: Regex::new(r#""\{\{([(\w)\-\+\.,@/\()\?=\*_\$])+(:([^}}])*)?\}\}""#)
                .unwrap(),
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
        let act_v = &serde_json::from_str::<Value>(&self.template_str).unwrap();
        // println!("{:#?}", act_v);

        let mut replaced_string = serde_json::to_string(act_v).unwrap();
        // println!("{:#?}", replaced_string);
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
                        println!("default value: {}", default_value.as_str());
                        //TODO: try to parse as string if it fails
                        let ay: Result<Value> =
                            serde_json::from_str::<Value>(default_value.as_str()).or_else(|_| {
                                let y = json!(default_value);
                                Ok(y)
                            });

                        return ay.expect(
                            format!("Invalid type for the default value of {}. Default Value are strings, boolean, number", key).as_str(),
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
                    Regex::new(format!(r#""\{{\{{{}(:)?\}}\}}""#, key).as_str()).unwrap();
                if let Some(y) = found.get_default_value() {
                    key_reg =
                        Regex::new(format!(r#""\{{\{{{}(:{})?\}}\}}""#, key, y).as_str()).unwrap();
                }

                let serialize = serde_json::to_string(&actual_value).unwrap();
                let des = serde_json::from_str::<Value>(&serialize).unwrap();
                // println!("{} {}", des.is_number(), des.is_boolean());
                replaced_string = key_reg
                    .replace(
                        replaced_string.as_str(),
                        serde_json::to_string(&actual_value).unwrap(),
                    )
                    .to_string();

                // println!("{}", replaced_string);
            });

        println!("{}", replaced_string);

        println!(
            "error: {:#?}",
            serde_json::from_str::<Value>(replaced_string.as_str())
        );

        y
    }
}
