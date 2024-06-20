use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RDBError;
use crate::schema::property_type::PropertyType;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class Property {
    readonly type: string;
    readonly version?: number;
    readonly primaryKey?: string;
    readonly items?: Property;
    readonly maxItems?: number;
    readonly minItems?: number;
    readonly maxLength?: number;
    readonly minLength?: number;
    readonly required?: string[];
    readonly properties?: {
        [name: string]: Property
    };
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Property   {
    #[serde(rename = "type")]
    pub(crate) property_type: PropertyType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) items: Option<Vec<Property>>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub(crate) max_items: Option<i32>,
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub(crate) min_items: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) properties: Option<HashMap<String, Property>>,

    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub(crate) max_length: Option<i32>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub(crate) min_length: Option<i32>,
}

#[wasm_bindgen]
impl Property {
    #[wasm_bindgen(getter, js_name="type")]
    pub fn property_type(&self) -> PropertyType {
        self.property_type
    }

    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Result<JsValue, JsValue> {
        Ok(
            to_value(&self.items)
            .map_err(|e| JsValue::from(RDBError::from(e)))?
        )
    }

    #[wasm_bindgen(getter, js_name="maxItems")]
    pub fn max_items(&self) -> Result<JsValue, JsValue> {
        Ok(
            to_value(&self.max_items)
                .map_err(|e| JsValue::from(RDBError::from(e)))?
        )
    }

    #[wasm_bindgen(getter, js_name="minItems")]
    pub fn min_items(&self) -> Result<JsValue, JsValue> {
        Ok(
            to_value(&self.min_items)
                .map_err(|e| JsValue::from(RDBError::from(e)))?
        )
    }

    #[wasm_bindgen(getter, js_name="maxlength")]
    pub fn max_length(&self) -> Result<JsValue, JsValue> {
        Ok(
            to_value(&self.max_length)
                .map_err(|e| JsValue::from(RDBError::from(e)))?
        )
    }

    #[wasm_bindgen(getter, js_name="minlength")]
    pub fn min_length(&self) -> Result<JsValue, JsValue> {
        Ok(
            to_value(&self.min_length)
                .map_err(|e| JsValue::from(RDBError::from(e)))?
        )
    }

    #[wasm_bindgen(getter, js_name="required")]
    pub fn required(&self) -> Result<JsValue, JsValue> {
        Ok(
            to_value(&self.required)
                .map_err(|e| JsValue::from(RDBError::from(e)))?
        )
    }

    #[wasm_bindgen(getter, js_name="properties")]
    pub fn properties(&self) -> Result<JsValue, JsValue> {
        Ok(
            to_value(&self.properties)
                .map_err(|e| JsValue::from(RDBError::from(e)))?
        )
    }



}