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
/**
 * Represents the type definition for a schema.
 */
export type SchemaType = {
    /**
     * The version of the schema.
     */
    readonly version: number;

    /**
     * The primary key of the schema.
     */
    readonly primaryKey: string;

    /**
     * The type of the schema.
     */
    readonly type: string;

    /**
     * An optional array of required fields.
     */
    readonly required?: string[];

    /**
     * An optional array of indexes.
     */
    readonly indexes?: string[];

    /**
     * The properties defined in the schema.
     */
    readonly properties: {
        [name: string]: Property;
    };
};

/**
 * Represents a schema, including its definition and related methods.
 *
 * @template T - The schema type.
 */
export class Schema<T extends SchemaType> {
    /**
     * The schema definition.
     */
    schema: Schema<T>;

    /**
     * Creates a new `Schema` instance from the provided definition.
     *
     * @template TS - The schema type.
     * @param {TS} definition - The schema definition.
     * @returns {Schema<TS>} The created `Schema` instance.
     */
    static create<TS extends SchemaType>(definition: TS): Schema<TS>;

    /**
     * The version of the schema.
     */
    readonly version: number;

    /**
     * The primary key of the schema.
     */
    readonly primaryKey: string;

    /**
     * The type of the schema.
     */
    readonly type: string;

    /**
     * An optional array of required fields.
     */
    readonly required?: string[];

    /**
     * An optional array of indexes.
     */
    readonly indexes?: string[];

    /**
     * The properties defined in the schema.
     */
    readonly properties: {
        [name in keyof T['properties']]: T['properties'][name];
    };

    /**
     * Converts the schema to a JSON representation.
     *
     * @returns {SchemaType} The JSON representation of the schema.
     */
    toJSON(): SchemaType;
}
"#;

#[derive(Serialize, Deserialize, Clone, Default)]
#[wasm_bindgen(skip_typescript)]
/// Represents the schema of a collection, including version, primary key, type, required fields, properties, and indexes.
pub struct Schema {
    /// The version of the schema.
    pub(crate) version: i32,
    /// The primary key of the schema.
    #[serde(rename = "primaryKey")]
    pub(crate) primary_key: String,
    /// The type of the schema.
    #[serde(rename = "type")]
    pub(crate) schema_type: String,
    /// The required fields in the schema, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Vec<String>>,
    /// The properties defined in the schema.
    pub(crate) properties: HashMap<String, Property>,
    /// The indexes defined in the schema, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) indexes: Option<Vec<String>>,
}

#[wasm_bindgen]
impl Schema {

    /// Creates a new `Schema` instance from a given `JsValue`.
    ///
    /// # Arguments
    ///
    /// * `schema` - A `JsValue` representing the schema.
    ///
    /// # Returns
    ///
    /// * `Result<Schema, JsValue>` - A result containing the new `Schema` instance or an error.
    #[wasm_bindgen]
    pub fn create(schema: JsValue) -> Result<Schema, JsValue> {
        Ok(
            from_value(schema)
                .map_err(|e| JsValue::from(RIDBError::from(e)))?
        )
    }

    /// Retrieves the version of the schema.
    ///
    /// # Returns
    ///
    /// * `i32` - The version of the schema.
    #[wasm_bindgen(getter, js_name="version")]
    pub fn get_version(&self) -> i32 {
        self.version
    }

    /// Retrieves the primary key of the schema.
    ///
    /// # Returns
    ///
    /// * `String` - The primary key of the schema.
    #[wasm_bindgen(getter, js_name="primaryKey")]
    pub fn get_primary_key(&self) -> String {
        self.primary_key.clone()
    }

    /// Retrieves the type of the schema.
    ///
    /// # Returns
    ///
    /// * `String` - The type of the schema.
    #[wasm_bindgen(getter, js_name="type")]
    pub fn get_schema_type(&self) -> String {
        self.schema_type.clone()
    }

    /// Retrieves the required fields of the schema, if any.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<String>>` - The required fields of the schema, if any.
    #[wasm_bindgen(getter, js_name="required")]
    pub fn get_required(&self) -> Option<Vec<String>> {
        self.required.clone()
    }

    /// Retrieves the indexes of the schema, if any.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<String>>` - The indexes of the schema, if any.
    #[wasm_bindgen(getter, js_name="indexes")]
    pub fn get_indexes(&self) -> Option<Vec<String>> {
        self.indexes.clone()
    }

    /// Retrieves the properties of the schema.
    ///
    /// # Returns
    ///
    /// * `Result<JsValue, JsValue>` - A result containing the properties as a `JsValue` or an error.
    #[wasm_bindgen(getter, js_name="properties")]
    pub fn get_properties(&self) -> Result<JsValue, JsValue> {
        to_value(&self.properties).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

}
