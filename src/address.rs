use std::collections::HashMap;

use mustache::Template;
use serde::{Deserialize, Serialize};
use serde_yml::Value;

use crate::SimpleAddressError;

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
    templates: HashMap<String, Template>,
}

impl SimpleDeliveryAddressFormatter {
    // Unwrap used here as the YAML is validated in the build.rs
    pub fn new() -> Self {
        let countries_template_data = serde_yml::from_slice::<Value>(&TEMPLATE_DATA).unwrap();
        let mut templates = HashMap::new();
        for (key, value) in countries_template_data.as_mapping().unwrap() {
            templates.insert(
                key.as_str().unwrap().to_lowercase(),
                mustache::compile_str(value.get("multi-line-template").unwrap().as_str().unwrap())
                    .unwrap(),
            );
        }
        return Self { templates };
    }

    pub fn generate_single_line_address<T: Serialize>(
        &self,
        country: &str,
        address_parts: &T,
    ) -> Result<String, SimpleAddressError> {
        Ok(multiline_string_to_single(
            self.generate_multi_line_address(country, address_parts)?,
        ))
    }

    pub fn generate_multi_line_address<T: Serialize>(
        &self,
        country: &str,
        address_parts: &T,
    ) -> Result<String, SimpleAddressError> {
        Ok(clean_multiline_string(
            self.templates
                .get(&country.to_lowercase())
                .ok_or(SimpleAddressError::CountryNotSupported(country.to_string()))?
                .render_to_string(address_parts)?,
        ))
    }
}

fn multiline_string_to_single(address: String) -> String {
    address.replace("\n", ", ")
}

fn clean_multiline_string(address: String) -> String {
    address
        .replace("\n\n", "\n")
        .replace("\n ", "\n")
        .trim()
        .to_string()
}
