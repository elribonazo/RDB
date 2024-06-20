use std::collections::HashMap;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::collection::Collection;
use crate::error::RIDBError;
use crate::storage::internals::storage_internal::{StorageModule};
use crate::storage::Storage;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class Database<T extends SchemaTypeRecord> {

    static create<TS extends SchemaTypeRecord>(
        schemas: TS,
        storage: StorageModule
    ): Promise<Database<TS>>;

    readonly collections: {
        [name in keyof T]: Collection<Schema<T[name]>>
    }
}
"#;


#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub struct Database {
    pub(crate) storage: Storage
}



#[wasm_bindgen]
impl Database {

    #[wasm_bindgen(getter)]
    pub fn collections(&self) -> Result<Object, JsValue> {
        let mut collections: HashMap<String, Collection> =
            self.storage.internals
            .iter()
            .map(|(key, internals)| {
                (key.clone(),Collection::from(key.clone(), internals.clone()))
            })
            .collect();

        let object = Object::new();
        for (key, collection) in collections {
            Reflect::set(
                &object,
                &JsValue::from_str(key.as_str()),
                &JsValue::from(collection)
            ).map_err(|e| JsValue::from(RIDBError::from("Failed to retrieve value")))?;

        }

        Ok(
            object
        )
    }

    #[wasm_bindgen]
    pub async fn create(
        schemas_map_js: Object,
        module: StorageModule
    ) -> Result<Database, JsValue> {
        if !schemas_map_js.is_object() {
            return Err(JsValue::from(RIDBError::from("Unexepcted object")));
        }
        let storage_internal_map_js = module.create_storage(&schemas_map_js.clone()).await;
        let storage =
                Storage::create(storage_internal_map_js.clone().into())
                    .map_err(|e| {
                    JsValue::from(RIDBError::from(e))
                })?;
        Ok(
            Database {
                storage
            }
        )
    }
}