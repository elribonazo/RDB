use js_sys::Object;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::query::Operation;
use crate::schema::Schema;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export class BaseStorage<T extends SchemaType> {
  free(): void;
  constructor(name: string, schema_type: any);
  readonly name: string;
  readonly schema: Schema<T>;
  abstract write(op: Operation<T>): Promise<Doc<T>>;
  abstract query(): Promise<void>;
  abstract findDocumentById(id: string): Promise<null>;
  abstract count(): Promise<number>;
  abstract remove(id: string): Promise<void>;
  abstract close(): Promise<void>;
}


export type SchemaTypeRecord = { [name: string]: SchemaType };

export type CreateStorage = <T extends SchemaTypeRecord = SchemaTypeRecord>(
    records: T
) => Promise<InternalsRecord>

export type StorageModule = {
    createStorage: CreateStorage
}
"#;

#[wasm_bindgen]
#[derive(Clone)]
pub struct BaseStorage {
    pub(crate) name: String,
    pub(crate) schema: Schema,
}

#[wasm_bindgen]
impl BaseStorage {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, schema_type: JsValue) -> Result<BaseStorage, JsValue> {
        let schema = Schema::create(schema_type)?;
        Ok(
            BaseStorage {
                name,
                schema
            }
        )
    }
    #[wasm_bindgen(method, getter = schema)]
    pub fn schema(&self) -> Schema {
        self.schema.clone()
    }

    #[wasm_bindgen(method, getter = name)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}




#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Default)]
    pub type StorageInternal;

    #[derive(Clone, Default)]
    pub type StorageModule;

    #[wasm_bindgen(method, js_name="createStorage")]
    pub async fn create_storage(this: &StorageModule, records: &Object) -> JsValue;

    #[wasm_bindgen(constructor)]
    pub fn new(name: &JsValue, schema: &JsValue) -> StorageInternal;

    #[wasm_bindgen(method, getter = schema)]
    pub fn schema(this: &StorageInternal) -> Schema;

    #[wasm_bindgen(method, getter = name)]
    pub fn name(this: &StorageInternal) -> String;

    #[wasm_bindgen(method, catch)]
    pub async fn write(this: &StorageInternal, op: Operation) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn query(this: &StorageInternal) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name="findDocumentById")]
    pub async fn findDocument_by_id(this: &StorageInternal) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn count(this: &StorageInternal) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn remove(this: &StorageInternal) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn close(this: &StorageInternal) -> Result<JsValue, JsValue>;
}
