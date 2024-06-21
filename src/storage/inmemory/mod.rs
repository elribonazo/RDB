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
}

pub trait StorageOperations {
    async fn write(&self, op: Operation) -> Result<JsValue, JsValue>;
    async fn query(&self) -> Result<JsValue, JsValue>;
    async fn find_document_by_id(&self) -> Result<JsValue, JsValue>;
    async fn count(&self) -> Result<JsValue, JsValue>;
    async fn remove(&self) -> Result<JsValue, JsValue>;
    async fn close(&self) -> Result<JsValue, JsValue>;
}


impl StorageOperations for InMemory {
    async fn write(&self, op: Operation) -> Result<JsValue, JsValue> {
        match op.op_type {
            OpType::CREATE => {
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
            base
        }
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
    pub async fn write_js(&self, op: Operation) -> Result<JsValue, JsValue> {
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
