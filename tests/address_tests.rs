use serde::Deserialize;
use simple_delivery_address_rs::{SimpleDeliveryAddress, SimpleDeliveryAddressFormatter};
use std::fs::{self, File};

#[derive(Deserialize, Debug)]
struct TestData {
    components: SimpleDeliveryAddress,
    expected_multiline: String,
    expected_singleline: String,
}

#[test]
fn run_all_test_cases() {
    let test_paths = fs::read_dir("simple-delivery-address/tests/address_formats")
        .unwrap()
        .filter_map(|x| {
            let path = x.unwrap().path();
            let extension = path.extension().unwrap();
            if extension == "yml" || extension == "yaml" {
                Some(path)
            } else {
                None
            }
        });

    let address_formatter = SimpleDeliveryAddressFormatter::new();

    for path in test_paths {
        let country = path.file_stem().unwrap().to_str().unwrap();

        let test_data: Vec<TestData> = serde_yml::from_reader(File::open(&path).unwrap()).unwrap();

        for test_scenario in test_data {
            let actual_multiline =
                address_formatter.generate_multi_line_address(country, &test_scenario.components);

            assert!(actual_multiline.is_ok());

            assert_eq!(
                actual_multiline.unwrap(),
                test_scenario.expected_multiline.trim()
            );
            let actual_singleline =
                address_formatter.generate_single_line_address(country, &test_scenario.components);

            assert!(actual_singleline.is_ok());

            assert_eq!(
                actual_singleline.unwrap(),
                test_scenario.expected_singleline.trim()
            )
        }
    }
}
