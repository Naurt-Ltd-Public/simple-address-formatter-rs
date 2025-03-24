use std::collections::HashMap;

use mustache::Template;
use serde::{Deserialize, Serialize};
use serde_yml::Value;

const TEMPLATE_DATA: &[u8] = include_bytes!("../test/countries.yaml");

#[derive(Serialize, Deserialize)]
pub struct SimpleDeliveryAddress {
    pub unit: Option<String>,
    pub house_name: Option<String>,
    pub street_number: Option<String>,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub county: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postalcode: Option<String>,
}

pub struct SimpleDeliveryAddressFormatter {
    templates: HashMap<String, AddressTemplates>,
}

// TODO, implement results etc.
impl SimpleDeliveryAddressFormatter {
    pub fn new() -> Self {
        let countries_template_data = serde_yml::from_slice::<Value>(&TEMPLATE_DATA).unwrap();

        let mut templates = HashMap::new();
        for (key, value) in countries_template_data.as_mapping().unwrap() {
            templates.insert(
                key.as_str().unwrap().to_lowercase(),
                AddressTemplates {
                    single_line: mustache::compile_str(
                        value.get("single-line").unwrap().as_str().unwrap(),
                    )
                    .unwrap(),
                    multi_line: mustache::compile_str(
                        value.get("multi-line").unwrap().as_str().unwrap(),
                    )
                    .unwrap(),
                },
            );
        }

        return Self { templates };
    }

    pub fn generate_single_line_address<T: Serialize>(
        &self,
        country: &str,
        address_parts: &T,
    ) -> Option<String> {
        return Some(
            self.templates
                .get(&country.to_lowercase())?
                .multi_line
                .render_to_string(address_parts)
                .unwrap()
                .replace("\n", ", "),
        );
    }

    pub fn generate_multi_line_address<T: Serialize>(
        &self,
        country: &str,
        address_parts: &T,
    ) -> Option<String> {
        return Some(
            self.templates
                .get(&country.to_lowercase())?
                .multi_line
                .render_to_string(address_parts)
                .unwrap(),
        );
    }
}

struct AddressTemplates {
    single_line: Template,
    multi_line: Template,
}
