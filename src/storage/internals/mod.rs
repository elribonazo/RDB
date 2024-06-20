pub mod storage_internal;

use js_sys::Reflect;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RDBError;
use crate::query::{Operation, OpType};
use crate::schema::property_type::PropertyType;
use crate::schema::Schema;
use crate::storage::internals::storage_internal::StorageInternal;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class Internals<T extends SchemaType> {
    readonly internal: StorageInternal<T>
    constructor(internal: StorageInternal<T>);
    readonly schema: T
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Default)]
pub struct Internals {
    pub(crate) schema: Schema,
    pub(crate) internal: StorageInternal
}

#[wasm_bindgen]
impl Internals {
    #[wasm_bindgen(constructor)]
    pub fn new(
        internal: StorageInternal
    ) -> Internals {
        let schema = internal.schema().clone();
        Internals {
            schema,
            internal
        }
    }

    fn ensure_primary_key(&self, document: JsValue) -> Result<JsValue, JsValue> {
        let key = self.schema.primary_key.clone();
        let doc_property = Reflect::get(
            &document,
            &JsValue::from(&key)
        ).map_err(|e| JsValue::from(RDBError::from(e)))?;
        let properties = self.schema.properties.clone();
        let primary_key_property = properties.get(&key)
            .ok_or(RDBError::from("Invalid Schema cannot find primaryKey field"))
            .map_err(|e| JsValue::from(e))?;
        let primary_key_type = primary_key_property.property_type();
        if doc_property.is_null() || doc_property.is_undefined() {
            if primary_key_type == PropertyType::String {
                Reflect::set(
                    &document,
                    &JsValue::from(&key),
                    &JsValue::from("12245")
                ).map_err(|e| JsValue::from(RDBError::from(e)))?;
            } else {
                Reflect::set(
                    &document,
                    &JsValue::from(&key),
                    &JsValue::from(12345)
                ).map_err(|e| JsValue::from(RDBError::from(e)))?;
            }
        }
        let doc_property = Reflect::get(
            &document,
            &JsValue::from(&key)
        ).map_err(|e| JsValue::from(RDBError::from(e)))?;
        if primary_key_type == PropertyType::String && !doc_property.is_string() {
            Err(JsValue::from(RDBError::from("Unexpected primary key should be a string")))
        } else if primary_key_type == PropertyType::Number && !doc_property.is_bigint() {
            Err(JsValue::from(RDBError::from("Unexepcted primary key should be number")))
        } else {
            Ok(document)
        }
    }

    pub fn validate_schema(&self, document_without_pk: JsValue) -> Result<JsValue, JsValue> {
        let document = self.ensure_primary_key(document_without_pk)?;
        let properties = self.schema.properties.clone();
        for (key, prop) in properties {
            let value = Reflect::get(&document, &JsValue::from_str(key.clone().as_str()))?;
            if value.is_undefined() {
                if let Some(required_fields) = &self.schema.required {
                    if required_fields.contains(&key) {
                        return Err(JsValue::from(RDBError::error(format!("Field {} is required", key))));
                    }
                }
            } else {
                if !self.is_type_correct(&value, prop.property_type) {
                    return Err(JsValue::from(RDBError::error(format!("Field {} should match type {:?}", key, prop.property_type))));
                }
            }
        }
        Ok(document)
    }

    pub fn is_type_correct(&self,value: &JsValue, prop_type: PropertyType) -> bool {
        match prop_type {
            PropertyType::String => value.is_string(),
            PropertyType::Number => value.as_f64().is_some(),
            PropertyType::Object => value.is_object(),
            PropertyType::Array => {
                if let Some(array) = value.dyn_ref::<js_sys::Array>() {
                    !array.is_array()

                } else {
                    false
                }
            },
            PropertyType::Boolean => value.is_falsy() || value.is_truthy(),
            _ => false,
        }
    }

    #[wasm_bindgen]
    pub async fn write(&self, document_without_pk: JsValue) -> Result<JsValue, JsValue> {
        let primary_key = self.schema.primary_key.clone();
        let document = self
            .validate_schema(document_without_pk)
            .map_err(|e| JsValue::from(RDBError::from("Could not add primary key")))?;

        let mut indexes = match self.schema.indexes.clone() {
            Some(mut existing) => {
                existing.push(primary_key.clone());
                existing
            },
            _ => {
                let mut new_index: Vec<String> = Vec::new();
                new_index.push(primary_key.clone());
                new_index
            }
        };

        let op = Operation {
            collection: self.internal.name().clone(),
            op_type: OpType::CREATE,
            data: document,
            indexes,
        };
        let result = self.internal.write(op).await;
        result.map_err(|e| JsValue::from(RDBError::from(e)))
    }

    #[wasm_bindgen]
    pub async fn query() {

    }
    #[wasm_bindgen]
    pub async fn find_document_by_id() {

    }
    #[wasm_bindgen]
    pub async fn count() {

    }
    #[wasm_bindgen]
    pub async fn remove() {

    }
    #[wasm_bindgen]
    pub async fn close() {

    }

}