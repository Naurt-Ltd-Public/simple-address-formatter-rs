use std::collections::BTreeMap;

use include_dir::{include_dir, Dir};
use serde_yml::Value;

static TEMPLATES_DIR: Dir =
    include_dir!("$CARGO_MANIFEST_DIR/simple-address-format/templates/address_formats");

fn main() {
    println!("Reading in template data and verifying.");
    let mut m = BTreeMap::new();

    for f in TEMPLATES_DIR.files() {
        let name = f.path().file_stem().unwrap().to_string_lossy().to_string(); // "AT"
        let text = f.contents_utf8().expect("utf-8");

        if let Ok(map) = serde_yml::from_str::<BTreeMap<String, Value>>(text) {
            for (k, v) in map {
                m.insert(
                    k.to_lowercase(),
                    (
                        mustache::compile_str(
                            &v.get("multiline_template").unwrap().as_str().unwrap(),
                        )
                        .unwrap(),
                        mustache::compile_str(
                            &v.get("singleline_template").unwrap().as_str().unwrap(),
                        )
                        .unwrap(),
                    ),
                );
            }
        } else {
            let v: Value = serde_yml::from_str(text).expect("yaml shape");
            m.insert(
                name.to_lowercase(),
                (
                    mustache::compile_str(&v.get("multiline_template").unwrap().as_str().unwrap())
                        .unwrap(),
                    mustache::compile_str(&v.get("singleline_template").unwrap().as_str().unwrap())
                        .unwrap(),
                ),
            );
        }
    }
}
