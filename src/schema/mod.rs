pub mod property_type;
pub mod property;

use std::collections::HashMap;
use std::hash::Hash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RIDBError;
use crate::schema::property::Property;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type SchemaType = {
    readonly version: number;
    readonly primaryKey: string;
    readonly type: string;
    readonly required?: string[];
    readonly indexes?: string[];
    readonly properties:{
        [name: string]: Property
    };
};
export class Schema<T extends SchemaType> {
    schema: Schema<T>
    static create<TS extends SchemaType>(definition: TS): Schema<TS>
    readonly version: number;
    readonly primaryKey: string;
    readonly type: string;
    readonly required?: string[];
    readonly indexes?: string[];
    readonly properties: {
        [name in keyof T['properties']]: T['properties'][name];
    };
    toJSON(): SchemaType;
}
"#;


#[derive(Serialize, Deserialize, Clone, Default)]
#[wasm_bindgen(skip_typescript)]
pub struct Schema {
    pub(crate) version: i32,
    #[serde(rename = "primaryKey")]
    pub(crate) primary_key: String,
    #[serde(rename = "type")]
    pub(crate) schema_type:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Vec<String>>,
    pub(crate) properties: HashMap<String, Property>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) indexes: Option<Vec<String>>,
}

#[wasm_bindgen]
impl Schema {

    #[wasm_bindgen]
    pub fn create(schema: JsValue) -> Result<Schema, JsValue> {
        Ok(
            from_value(schema)
                .map_err(|e| JsValue::from(RIDBError::from(e)))?
        )
    }

    #[wasm_bindgen(getter, js_name="version")]
    pub fn get_version(&self) -> i32 {
        self.version
    }

    #[wasm_bindgen(getter, js_name="primaryKey")]
    pub fn get_primary_key(&self) -> String {
        self.primary_key.clone()
    }

    #[wasm_bindgen(getter, js_name="type")]
    pub fn get_schema_type(&self) -> String {
        self.schema_type.clone()
    }

    #[wasm_bindgen(getter, js_name="required")]
    pub fn get_required(&self) -> Option<Vec<String>> {
        self.required.clone()
    }

    #[wasm_bindgen(getter, js_name="indexes")]
    pub fn get_indexes(&self) -> Option<Vec<String>> {
        self.indexes.clone()
    }

    #[wasm_bindgen(getter, js_name="properties")]
    pub fn get_properties(&self) -> Result<JsValue, JsValue> {
        to_value(&self.properties).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

}