

extern crate wasm_bindgen_test;
use crate::schema::property::Property;
use crate::schema::property_type::PropertyType;
use wasm_bindgen_test::{wasm_bindgen_test};

#[cfg(feature = "browser")]
use wasm_bindgen_test::{wasm_bindgen_test_configure};

#[cfg(feature = "browser")]
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_property_type() {
    let prop = Property {
        property_type: PropertyType::String,
        items: None,
        max_items: Some(10),
        min_items: Some(1),
        max_length: None,
        min_length: None,
        required: None,
        properties: None,
    };

    assert_eq!(prop.property_type(), PropertyType::String);
}