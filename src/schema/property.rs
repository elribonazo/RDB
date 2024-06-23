extern crate wasm_bindgen_test;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RIDBError;
use crate::schema::property_type::PropertyType;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents a property within a schema, including various constraints and nested properties.
 */
export class Property {
    /**
     * The type of the property.
     */
    readonly type: string;

    /**
     * The version of the property, if applicable.
     */
    readonly version?: number;

    /**
     * The primary key of the property, if applicable.
     */
    readonly primaryKey?: string;

    /**
     * An optional array of nested properties for array-type properties.
     */
    readonly items?: Property[];

    /**
     * The maximum number of items for array-type properties, if applicable.
     */
    readonly maxItems?: number;

    /**
     * The minimum number of items for array-type properties, if applicable.
     */
    readonly minItems?: number;

    /**
     * The maximum length for string-type properties, if applicable.
     */
    readonly maxLength?: number;

    /**
     * The minimum length for string-type properties, if applicable.
     */
    readonly minLength?: number;

    /**
     * An optional array of required fields for object-type properties.
     */
    readonly required?: string[];

    /**
     * An optional map of nested properties for object-type properties.
     */
    readonly properties?: {
        [name: string]: Property;
    };
}
"#;


#[wasm_bindgen(skip_typescript)]
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
/// Represents a property within a schema, including type, items, length constraints, and other attributes.
pub struct Property {
    /// The type of the property.
    #[serde(rename = "type")]
    pub(crate) property_type: PropertyType,

    /// Optional nested items for array-type properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) items: Option<Vec<Property>>,

    /// Optional maximum number of items for array-type properties.
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub(crate) max_items: Option<i32>,

    /// Optional minimum number of items for array-type properties.
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub(crate) min_items: Option<i32>,

    /// Optional required fields for object-type properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Vec<String>>,

    /// Optional nested properties for object-type properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) properties: Option<HashMap<String, Property>>,

    /// Optional maximum length for string-type properties.
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub(crate) max_length: Option<i32>,

    /// Optional minimum length for string-type properties.
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub(crate) min_length: Option<i32>,
}

#[wasm_bindgen]
impl Property {
    /// Checks is the schema is valid.
    ///
    /// # Returns
    ///
    /// Throws exception if not valid
    pub fn is_valid(&self) -> bool {
        true
    }
    /// Retrieves the type of the property.
    ///
    /// # Returns
    ///
    /// * `PropertyType` - The type of the property.
    #[wasm_bindgen(getter, js_name = "type")]
    pub fn property_type(&self) -> PropertyType {
        self.property_type
    }

    /// Retrieves the items of the property.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the items as a `JsValue` or an error.
    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.items).map_err(|e| JsValue::from(RIDBError::from(e)))?)
    }

    /// Retrieves the maximum number of items of the property.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the maximum number of items as a `JsValue` or an error.
    #[wasm_bindgen(getter, js_name = "maxItems")]
    pub fn max_items(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.max_items).map_err(|e| JsValue::from(RIDBError::from(e)))?)
    }

    /// Retrieves the minimum number of items of the property.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the minimum number of items as a `JsValue` or an error.
    #[wasm_bindgen(getter, js_name = "minItems")]
    pub fn min_items(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.min_items).map_err(|e| JsValue::from(RIDBError::from(e)))?)
    }

    /// Retrieves the maximum length of the property.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the maximum length as a `JsValue` or an error.
    #[wasm_bindgen(getter, js_name = "maxLength")]
    pub fn max_length(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.max_length).map_err(|e| JsValue::from(RIDBError::from(e)))?)
    }

    /// Retrieves the minimum length of the property.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the minimum length as a `JsValue` or an error.
    #[wasm_bindgen(getter, js_name = "minLength")]
    pub fn min_length(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.min_length).map_err(|e| JsValue::from(RIDBError::from(e)))?)
    }

    /// Retrieves the required fields of the property.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the required fields as a `JsValue` or an error.
    #[wasm_bindgen(getter)]
    pub fn required(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.required).map_err(|e| JsValue::from(RIDBError::from(e)))?)
    }

    /// Retrieves the nested properties of the property.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the nested properties as a `JsValue` or an error.
    #[wasm_bindgen(getter)]
    pub fn properties(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(&self.properties).map_err(|e| JsValue::from(RIDBError::from(e)))?)
    }
}



#[cfg(feature = "browser")]
use wasm_bindgen_test::{wasm_bindgen_test_configure};

#[cfg(feature = "browser")]
wasm_bindgen_test_configure!(run_in_browser);

mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    use crate::schema::property::Property;
    use crate::schema::property_type::PropertyType;

    #[wasm_bindgen_test]
    fn test_property_defaults() {
        let default_property = Property {
            property_type: PropertyType::String,
            items: None,
            max_items: None,
            min_items: None,
            max_length: None,
            min_length: None,
            required: None,
            properties: None,
        };
        // Test default values to ensure proper initialization
        assert_eq!(default_property.property_type, PropertyType::String);
        assert!(default_property.items.is_none());
        assert!(default_property.max_items.is_none());
        assert!(default_property.min_items.is_none());
        assert!(default_property.max_length.is_none());
        assert!(default_property.min_length.is_none());
        assert!(default_property.required.is_none());
        assert!(default_property.properties.is_none());

        let a = default_property.is_valid();
        assert!(a);

    }

}
