use std::collections::HashMap;

use mustache::Template;
use serde::{Deserialize, Serialize};
use serde_yml::Value;

use crate::SimpleAddressError;

const TEMPLATE_DATA: &[u8] =
    include_bytes!("../simple-address-format/templates/address_formats/countries.yaml");

const MULTILINE_DELIMITER: &'static str = "\n";
const SINGLELINE_DELIMITER: &'static str = ", ";

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Simple format to store address components
///
/// Most address fields will map to one of the fields below. For instance a neighborhood
/// or borough of a city becomes a locality.
pub struct SimpleAddressFormat {
    /// Flats, units, floors, apartments, etc.
    pub unit: Option<String>,
    /// Name of building, house, etc.
    pub house_name: Option<String>,
    /// Number on street. Not always numerical.
    pub street_number: Option<String>,
    /// Name of the street.
    pub street_name: Option<String>,
    /// Sub-region of city, or dependent town etc.
    pub locality: Option<String>,
    /// Postal city or town
    pub city: Option<String>,
    /// Country. Could also be a province or other administrative region
    pub county: Option<String>,
    /// State. For UK, this can include England, Scotland, Wales.
    pub state: Option<String>,
    /// The country or territory.
    pub country: Option<String>,
    /// Post code, ZipCode etc.
    pub postalcode: Option<String>,
}

/// Formats address parts into country specific multiline or singleline address.
///
/// This stores pre-compiled templates for speedy run time formatting.
pub struct SimpleAddressFormatter {
    templates: HashMap<String, AddressTemplates>,
}

impl SimpleAddressFormatter {
    /// Create a new address formatter
    ///
    /// Available address format templates are compiled and stored in a hashmap with
    /// the country's two letter ISO code used to access it.
    ///
    /// Unwrap used here as the YAML is validated in the build.rs
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

    /// Format address parts into a singleline address, separated by commas.
    ///
    /// # Arguments
    /// * `country`: Two letter ISO code of country to use to format
    /// * `address_parts`: A serializable struct that contains the address fields to be serialized. Should align with [SimpleAddressFormat].
    pub fn generate_singleline_address<T: Serialize>(
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

    /// Format address parts into a multiline address, separated by `\n`
    ///
    /// # Arguments
    /// * `country`: Two letter ISO code of country to use to format
    /// * `address_parts`: A serializable struct that contains the address fields to be serialized. Should align with [SimpleAddressFormat].
    pub fn generate_multiline_address<T: Serialize>(
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
                if va == "-" {
                    // Added to handle empty portions of Brazil addresses.
                    None
                } else {
                    Some(va.to_owned() + SINGLELINE_DELIMITER)
                }
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
            if va.is_empty() {
                None
            } else {
                let new_va = va
                    .trim_matches(|c| c == ',' || c == ' ' || c == '-')
                    .to_owned();

                if new_va.is_empty() {
                    None
                } else {
                    Some(new_va + MULTILINE_DELIMITER)
                }
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
