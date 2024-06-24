use js_sys::{Object, Reflect};
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
    pub(crate) by_index: Object
}

pub trait StorageOperations {
    fn add_index(&mut self, name: String, row: JsValue);
    async fn write(&mut self, op: &Operation) -> Result<JsValue, JsValue>;
    async fn query(&self) -> Result<JsValue, JsValue>;
    async fn find_document_by_id(&self) -> Result<JsValue, JsValue>;
    async fn count(&self) -> Result<JsValue, JsValue>;
    async fn remove(&self) -> Result<JsValue, JsValue>;
    async fn close(&self) -> Result<JsValue, JsValue>;
}

impl StorageOperations for InMemory {
    fn add_index(&mut self, name: String, row: JsValue) {
        let index_key = JsValue::from(name);
        let primary_key = JsValue::from(&self.base.schema.primary_key);
        let primary_key_value = Reflect::get(&row, &JsValue::from(&primary_key)).expect("Cannot extract primary key");
        let obj:Object = match Reflect::get(&self.by_index, &index_key) {
            Ok(object) => match object.is_undefined() || object.is_null() {
                true => Object::new(),
                false => Object::from(object)
            },
            Err(_) => Object::new()
        };
        Reflect::set(
            &obj,
            &primary_key_value,
            &row
        ).expect("Cannot add row to object");
        Reflect::set(
            &self.by_index,
            &index_key,
            &obj
        ).expect("Cannot restore index documents");
    }

    async fn write(&mut self, op: &Operation) -> Result<JsValue, JsValue> {
        match &op.op_type {
            OpType::CREATE => {
                let indexes = &op.indexes;
                for index in indexes {
                    let collection_index = format!("{}+{}", self.base.name, index);
                    self.add_index(collection_index, op.data.clone());
                }
                Ok(op.data.clone())
            },
            _ => Err(JsValue::from_str("Optype is not supported"))
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
    pub fn new(name: &str, schema_type: JsValue) -> Result<InMemory, JsValue> {
        let base_res = BaseStorage::new(
            name.to_string(),
            schema_type
        );
        match base_res {
            Ok(base) => Ok(
                InMemory {
                    base,
                    by_index: Object::new()
                }
            ),
            Err(e) => Err(e)
        }
    }

    #[wasm_bindgen(getter)]
    pub fn by_index(&self) -> Result<JsValue, JsValue> {
        let by_index_object = self.by_index.clone();
        Ok(
            JsValue::from(by_index_object)
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
    pub async fn write_js(&mut self, op: &Operation) -> Result<JsValue, JsValue> {
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
