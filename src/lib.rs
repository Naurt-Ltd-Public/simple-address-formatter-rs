mod address;
mod error;

pub use address::{SimpleDeliveryAddress, SimpleDeliveryAddressFormatter};
pub use error::SimpleAddressError;

#[test]
fn test_simple_delivery_address() {
    let formatter = SimpleDeliveryAddressFormatter::new();
    let test_data = SimpleDeliveryAddress {
        unit: Some("flat 2".to_string()),
        house_name: Some("House Of Lords".to_string()),
        street_number: None, // Some("10".to_string()),
        street_name: Some("Rectory Road".to_string()),
        city: None, // Some("Beckenham".to_string()),
        county: Some("Greater London".to_string()),
        state: Some("England".to_string()),
        country: None, //Some("United Kingdom".to_string()),
        postalcode: Some("BR3 1HZ".to_string()),
    };

    println!("----------Multiline---------");
    let full_address = formatter
        .generate_multi_line_address("GB", &test_data)
        .unwrap();

    println!("{}", full_address);
    println!("----------------------")
}

#[test]
fn test_generic_struct() {
    use serde::Serialize;

    let formatter = SimpleDeliveryAddressFormatter::new();

    #[derive(Serialize)]
    struct SomeAddressData {
        #[serde(rename = "unit")]
        pub my_company_unit: String,
        pub house_name: Option<String>,
        pub street_number: Option<String>,
        pub street_name: Option<String>,
        pub city: Option<String>,
        pub county: Option<String>,
        pub state: Option<String>,
        pub country: Option<String>,
        pub postalcode: Option<String>,
    }

    let test_data = SomeAddressData {
        my_company_unit: "".to_string(),
        house_name: Some("House Of Lords".to_string()),
        street_number: None, // Some("10".to_string()),
        street_name: Some("Rectory Road".to_string()),
        city: None, // Some("Beckenham".to_string()),
        county: Some("Greater London".to_string()),
        state: Some("England".to_string()),
        country: Some("United Kingdom".to_string()),
        postalcode: Some("BR3 1HZ".to_string()),
    };

    let full_address = formatter
        .generate_multi_line_address("GB", &test_data)
        .unwrap();

    println!("------ Multiline ------");
    println!("{}", full_address);
    println!("----- SIngle line-----");
    let full_address = formatter
        .generate_single_line_address("GB", &test_data)
        .unwrap();
    println!("{}", full_address);
}
