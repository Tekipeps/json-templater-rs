#[derive(Clone, Debug, Default)]
pub struct Match {
    key: String,
    default_value: Option<String>,
}
impl Match {
    pub fn convert_string_to_match(str: String) -> Match {
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

    pub fn get_default_value(&self) -> Option<&String> {
        self.default_value.as_ref()
    }
    pub fn get_key(&self) -> &str {
        self.key.as_str()
    }
}
