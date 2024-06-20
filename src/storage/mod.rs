pub mod internals;

use std::collections::HashMap;
use js_sys::{ Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RIDBError;
use crate::storage::internals::{Internals};
use crate::storage::internals::storage_internal::StorageInternal;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type InternalsRecord = { [name: string]: BaseStorage<SchemaType> };
export class Storage<T extends InternalsRecord> {
    static create<
        TS extends InternalsRecord = InternalsRecord
    >(internals: TS): Storage<TS>
    readonly internals: {
        [name in keyof T]: T[name]
    }
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone)]
pub struct Storage {
    pub(crate) internals: HashMap<String, Internals>
}

#[wasm_bindgen]
impl Storage {

    #[wasm_bindgen]
    pub fn create(
        storages_map_js: Object,
    ) -> Result<Storage, JsValue> {

        if !storages_map_js.is_object() {
            return Err(JsValue::from(RIDBError::from("Unexepcted object")));
        }

        let keys = Object::keys(
            &storages_map_js.clone()
        ).into_iter();

        let mut storages: HashMap<String, StorageInternal> = HashMap::new();
        for key in keys {
            let key_string = key.as_string().unwrap();
            let value = Reflect::get(&storages_map_js.clone(), &key)
                .map_err(|e| JsValue::from(RIDBError::from("Failed to retrieve value")))?;
            storages.insert(key_string, value.clone().into());
        }

        let storages_mounted: HashMap<String, Internals> = storages
            .iter()
            .map(|(name, storage_internal)| {
                (name.clone(), Internals::new(storage_internal.clone()))
            })
            .collect::<HashMap<String, Internals>>();

        Ok(
            Storage {
                internals:storages_mounted
            }
        )
    }
}