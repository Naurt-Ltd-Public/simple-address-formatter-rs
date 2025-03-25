use std::collections::HashMap;

use serde_yml::Value;

const TEMPLATE_DATA: &[u8] = include_bytes!("test/countries.yaml");

fn main() {
    println!("Reading in template data and verifying.");
    let countries_template_data = serde_yml::from_slice::<Value>(&TEMPLATE_DATA).unwrap();
    let mut templates = HashMap::new();
    for (key, value) in countries_template_data.as_mapping().unwrap() {
        templates.insert(
            key.as_str().unwrap().to_lowercase(),
            mustache::compile_str(value.get("multi-line-template").unwrap().as_str().unwrap())
                .unwrap(),
        );
    }
}
