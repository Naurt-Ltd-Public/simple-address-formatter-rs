use std::collections::HashMap;

use mustache::Template;
use serde::{Deserialize, Serialize};
use serde_yml::Value;

use crate::SimpleAddressError;

const TEMPLATE_DATA: &[u8] =
    include_bytes!("../simple-delivery-address/templates/address_formats/countries.yaml");

const MULTILINE_DELIMITER: &'static str = "\n";
const SINGLELINE_DELIMITER: &'static str = ", ";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleDeliveryAddress {
    pub unit: Option<String>,
    pub house_name: Option<String>,
    pub street_number: Option<String>,
    pub street_name: Option<String>,
    pub locality: Option<String>,
    pub city: Option<String>,
    pub county: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postalcode: Option<String>,
}

pub struct SimpleDeliveryAddressFormatter {
    templates: HashMap<String, AddressTemplates>,
}

impl SimpleDeliveryAddressFormatter {
    // Unwrap used here as the YAML is validated in the build.rs
    pub fn new() -> Self {
        let countries_template_data = serde_yml::from_slice::<Value>(&TEMPLATE_DATA).unwrap();
        let mut templates = HashMap::new();
        for (key, value) in countries_template_data.as_mapping().unwrap() {
            templates.insert(
                key.as_str().unwrap().to_lowercase(),
                AddressTemplates {
                    singleline: mustache::compile_str(
                        value.get("singleline_template").unwrap().as_str().unwrap(),
                    )
                    .unwrap(),
                    multiline: mustache::compile_str(
                        value.get("multiline_template").unwrap().as_str().unwrap(),
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
    ) -> Result<String, SimpleAddressError> {
        Ok(clean_singleline_string(
            self.templates
                .get(&country.to_lowercase())
                .ok_or(SimpleAddressError::CountryNotSupported(country.to_string()))?
                .singleline
                .render_to_string(address_parts)?,
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
                .multiline
                .render_to_string(address_parts)?,
        ))
    }
}

fn clean_singleline_string(address: String) -> String {
    address
        .split(SINGLELINE_DELIMITER)
        .filter_map(|x| {
            let va = x.trim();
            if !va.is_empty() {
                Some(va.to_owned() + SINGLELINE_DELIMITER)
            } else {
                None
            }
        })
        .collect::<String>()
        .trim_end_matches(SINGLELINE_DELIMITER)
        .to_string()
}

fn clean_multiline_string(address: String) -> String {
    address
        .split(MULTILINE_DELIMITER)
        .filter_map(|x| {
            let va = x.trim();
            if !va.is_empty() {
                Some(va.to_owned() + MULTILINE_DELIMITER)
            } else {
                None
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

struct AddressTemplates {
    singleline: Template,
    multiline: Template,
}
