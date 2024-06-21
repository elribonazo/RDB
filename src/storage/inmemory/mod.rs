use std::collections::HashMap;
use js_sys::Reflect;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::query::{Operation, OpType};
use crate::schema::Schema;
use crate::storage::internals::base_storage::BaseStorage;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Represents an in-memory storage system extending the base storage functionality.
 *
 * @template T - The schema type.
 */
export class InMemory<T extends SchemaType> extends BaseStorage<T> {
    /**
     * Frees the resources used by the in-memory storage.
     */
    free(): void;
}
"#;


#[wasm_bindgen(skip_typescript)]
pub struct InMemory{
    pub(crate) base: BaseStorage,
    pub(crate) by_index: HashMap<String, Vec<JsValue>>
}

pub trait StorageOperations {
    fn add_index(&mut self, name: String, row: JsValue);
    async fn write(&mut self, op: Operation) -> Result<JsValue, JsValue>;
    async fn query(&self) -> Result<JsValue, JsValue>;
    async fn find_document_by_id(&self) -> Result<JsValue, JsValue>;
    async fn count(&self) -> Result<JsValue, JsValue>;
    async fn remove(&self) -> Result<JsValue, JsValue>;
    async fn close(&self) -> Result<JsValue, JsValue>;
}

impl StorageOperations for InMemory {
    fn add_index(&mut self, name: String, row: JsValue) {
        let exists = self.by_index.get(&name);
        if let Some(mut index) = exists {
            let mut cloned = index.clone();
            cloned.push(row);
            self.by_index.insert(name, cloned);
        } else {
            self.by_index.insert(name, vec![row]);
        }
    }

    async fn write(&mut self, op: Operation) -> Result<JsValue, JsValue> {
        match op.op_type {
            OpType::CREATE => {
                let indexes = op.indexes;
                for index in indexes {
                    let index_val = Reflect::get(
                        &op.data,
                        &JsValue::from_str(index.as_str())
                    ).unwrap();
                    let collection_index = format!("{}+{}", self.base.name, index);
                    self.add_index(collection_index, index_val);
                }
                Ok(op.data)
            },
            _ => {
                Err(JsValue::from_str("Optype is not supported"))
            }
        }
    }

    async fn query(&self) -> Result<JsValue, JsValue> {
        todo!()
    }

    async  fn find_document_by_id(&self) -> Result<JsValue, JsValue> {
        todo!()
    }

    async fn count(&self) -> Result<JsValue, JsValue> {
        todo!()
    }

    async fn remove(&self) -> Result<JsValue, JsValue> {
        todo!()
    }

    async fn close(&self) -> Result<JsValue, JsValue> {
        todo!()
    }
}

#[wasm_bindgen]
impl InMemory {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, schema_type: JsValue) -> InMemory {
        let base = BaseStorage::new(
            name.to_string(),
            schema_type
        ).unwrap();
        InMemory {
            base,
            by_index: HashMap::new()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn by_index(&self) -> Result<JsValue, JsValue> {
        let hash_map: HashMap<String, Vec<String>> = HashMap::new();
        Ok(
            to_value(&hash_map).unwrap()
        )
    }

    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> Schema {
        self.base.schema.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.base.name.clone()
    }

    #[wasm_bindgen(js_name = "write")]
    pub async fn write_js(&mut self, op: Operation) -> Result<JsValue, JsValue> {
        self.write(op).await
    }

    #[wasm_bindgen(js_name = "query")]
    pub async fn query_js(&self) -> Result<JsValue, JsValue> {
        self.query().await
    }

    #[wasm_bindgen(js_name = "findDocumentById")]
    pub async fn find_document_by_id_js(&self) -> Result<JsValue, JsValue> {
        self.find_document_by_id().await
    }

    #[wasm_bindgen(js_name = "count")]
    pub async fn count_js(&self) -> Result<JsValue, JsValue> {
        self.count().await
    }

    #[wasm_bindgen(js_name = "remove")]
    pub async fn remove_js(&self) -> Result<JsValue, JsValue> {
        self.remove().await
    }

    #[wasm_bindgen(js_name = "close")]
    pub async fn close_js(&self) -> Result<JsValue, JsValue> {
        self.close().await
    }
}
