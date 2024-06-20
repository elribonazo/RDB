use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::schema::Schema;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export abstract class StorageInternal<T extends SchemaType> {
    abstract write(op: Operation<T>): Promise<Doc<T>>;
    abstract query(): Promise<void>;
    abstract findDocumentById(id: string): Promise<null>;
    abstract count(): Promise<number>;
    abstract remove(id: string): Promise<void>;
    abstract close(): Promise<void>;
}
export class BaseStorage<T extends SchemaType> extends StorageInternal<T>{
    free(): void;
    constructor(name: string, schema_type: any);
    readonly name: string;
    readonly schema: Schema<T>;
    close(): Promise<void>;
    count(): Promise<number>;
    findDocumentById(id: string): Promise<null>;
    query(): Promise<void>;
    remove(id: string): Promise<void>;
    write(op: Operation<T>): Promise<Doc<T>>;
}
"#;


#[wasm_bindgen(skip_typescript)]
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

