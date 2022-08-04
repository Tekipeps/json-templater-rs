use crate::matches::Match;
use regex::{Error, Regex};
use serde_json::{de, Result, Value};
use std::collections::{hash_map, HashMap};

pub struct Template {
    template_str: String,
    params: HashMap<String, Value>,
}

impl Template {
    pub fn new(s: &str) -> Self {
        let y: Template = Template {
            template_str: s.to_string(),
            params: HashMap::new(),
        };
        y
    }
    fn parse(&self) -> Vec<Match> {
        let reg: Regex = Regex::new(r#"\{\{([(\w)\-\+\.,@/\()\?=\*_\$])+(:(.)*)?\}\}"#).unwrap();

        //validate json
        serde_json::from_str::<Value>(&self.template_str).unwrap();

        let matches = reg
            .captures_iter(&self.template_str)
            .map(|a| {
                let b = a.get(0).unwrap().as_str();
                Match::convert_string_to_match(b.to_string())
            })
            .collect();

        matches
    }
}
