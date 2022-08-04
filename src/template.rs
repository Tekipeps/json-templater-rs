use crate::matches::Match;
use regex::{Regex};
use serde_json::{json, Result, Value};
use std::collections::HashMap;

pub struct Template {
    template_str: String,
    regex_pattern: Regex,
}

impl Template {
    pub fn new(s: &str) -> Self {
        let y: Template = Template {
            template_str: s.to_string(),
            regex_pattern: Regex::new(r#""\{\{([(\w)\-\+\.,@/\()\?=\*_\$])+(:([^}}])*)?\}\}""#)
                .unwrap(),
        };
        y
    }

    pub fn apply(&self, params: HashMap<&str, Value>) -> Result<Value> {
 
        let act_v = &serde_json::from_str::<Value>(&self.template_str).unwrap();
 
        let mut replaced_string = serde_json::to_string(act_v).unwrap();
 
        self
            .regex_pattern
            .captures_iter( &replaced_string.clone())
            .for_each(|found| {
                let found = found
                    .get(0)
                    .expect("Unable to extract from template ")
                    .as_str();
                let found = Match::convert_string_to_match(found.to_string());
                let key = found.get_key();

                let opt_actual_value = params.get(key).cloned();
                let actual_value = opt_actual_value.unwrap_or_else(|| {
                    if let Some(default_value) = found.get_default_value() {
                        let ay: Result<Value> =
                            serde_json::from_str::<Value>(default_value.as_str()).or_else(|_| {
                                let y = json!(default_value);
                                Ok(y)
                            });

                        return ay.unwrap_or_else(|_| panic!("Invalid type for the default value of {}. Default Value are strings, boolean, number", key));
                    }
                    panic!(
                        "{} has no default value. Add a value in params or a default value!",
                        key
                    );
                });

                // TODO: escape key names before regex check
                let mut key_reg =
                    Regex::new(format!(r#""\{{\{{{}(:)?\}}\}}""#, key).as_str()).unwrap();
                if let Some(y) = found.get_default_value() {
                    key_reg =
                        Regex::new(format!(r#""\{{\{{{}(:{})?\}}\}}""#, key, y).as_str()).unwrap();
                }

                replaced_string = key_reg
                    .replace(
                        replaced_string.as_str(),
                        serde_json::to_string(&actual_value).unwrap(),
                    )
                    .to_string();

                
            });

            let y = serde_json::from_str::<Value>(replaced_string.as_str());
            y
        
    }
}
