use serde_json::{Result, Value};
use std::collections::HashMap;
struct Template {
    template_str: String,
    params: HashMap<String, Value>,
}

impl Template {
    // fn new() -> Self {

    // }
}
#[cfg(test)]
mod tests {
    // use crate::meh;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
