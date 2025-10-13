use std::collections::BTreeMap;

use include_dir::{include_dir, Dir};
use mustache::Template;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::SimpleAddressError;

static TEMPLATES_DIR: Dir =
    include_dir!("$CARGO_MANIFEST_DIR/simple-address-format/templates/address_formats");

#[derive(Debug, Deserialize)]
struct FileTpl {
    multiline_template: String,
    singleline_template: String,
}

struct AddressTemplates {
    singleline: Template,
    multiline: Template,
}

static ALL: Lazy<BTreeMap<String, AddressTemplates>> = Lazy::new(|| {
    let mut m = BTreeMap::new();
    for f in TEMPLATES_DIR.files() {
        let name = f.path().file_stem().unwrap().to_string_lossy().to_string(); // "AT"
        let text = f.contents_utf8().expect("utf-8");
        // allow both shapes: {AT: {...}} or {...}
        if let Ok(map) = serde_yml::from_str::<BTreeMap<String, FileTpl>>(text) {
            for (k, v) in map {
                m.insert(
                    k.to_lowercase(),
                    AddressTemplates {
                        multiline: mustache::compile_str(&v.multiline_template).unwrap(),
                        singleline: mustache::compile_str(&v.singleline_template).unwrap(),
                    },
                );
            }
        } else {
            let v: FileTpl = serde_yml::from_str(text).expect("yaml shape");
            m.insert(
                name.to_lowercase(),
                AddressTemplates {
                    multiline: mustache::compile_str(&v.multiline_template).unwrap(),
                    singleline: mustache::compile_str(&v.singleline_template).unwrap(),
                },
            );
        }
    }
    m
});

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
pub struct SimpleAddressFormatter;

impl SimpleAddressFormatter {
    /// Create a new address formatter
    ///
    /// Available address format templates are compiled and stored in a hashmap with
    /// the country's two letter ISO code used to access it.
    ///
    /// Unwrap used here as the YAML is validated in the build.rs
    pub fn new() -> Self {
        let _a = &ALL;
        return Self;
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
            ALL.get(&country.to_lowercase())
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
            ALL.get(&country.to_lowercase())
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
