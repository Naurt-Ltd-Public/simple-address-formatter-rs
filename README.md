# Simple Address Formatter Rust

A rust implementation to format addresses using the templates found in the [Simple Address Format](https://github.com/Naurt-Ltd-Public/simple-address-format) repository.

Formats address parts/components into a multiline or singleline address.

## Examples

**Using the Naurt type**

```rust
use simple_address_formatter::{SimpleAddressFormat, SimpleAddressFormatter};

let address_formatter = SimpleAddressFormatter::new();

let my_address = SimpleAddressFormat {
    house_name: Some("Challenger Court".to_string()),
    street_number: Some("49".to_string()),
    street_name: Some("Wallis Avenue".to_string()),
    unit: Some("Flat 4".to_string()),
    city: Some("Maidstone".to_string()),
    postalcode: Some("ME15 9HS".to_string()),
    state: Some("England".to_string()),
    country: Some("United Kingdom".to_string()),
    locality: None,
    county: None,
};

let multiline_address = address_formatter
    .generate_multiline_address("gb", &my_address)
    .unwrap();
let singleline_address = address_formatter
    .generate_multiline_address("gb", &my_address)
    .unwrap();

println!("{}", multiline_address);
println!("{}", singleline_address);
```

**Using a custom type**

```rust
use serde::Serialize;
use simple_address_formatter::SimpleAddressFormatter;

#[derive(Serialize)]
struct MyAddressFormat {
    unit: String,
    street_number: String,
    street_name: String,
    #[serde(rename = "city")]
    town: String,
}

let address_formatter = SimpleAddressFormatter::new();

let my_address = MyAddressFormat {
    unit: "1".to_string(),
    street_number: "23".to_string(),
    street_name: "Wide Street".to_string(),
    town: "New York".to_string(),
};

let multiline_address = address_formatter
    .generate_multiline_address("us", &my_address)
    .unwrap();
let singleline_address = address_formatter
    .generate_multiline_address("us", &my_address)
    .unwrap();

println!("{}", multiline_address);
println!("{}", singleline_address);
```
