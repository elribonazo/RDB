use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RIDBError;
use crate::schema::Schema;
use crate::storage::internals::Internals;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type ExtractType<T extends string> =
    T extends 'string' ? string:
    T extends 'number' ? number:
    T extends 'boolean' ? boolean:
    T extends 'object' ? object:
    T extends 'array' ? Array<any>:
    never;

export type Doc<
    T extends SchemaType
> =  {
    [name in keyof T['properties']]: ExtractType<T['properties'][name]['type']>
}

export class Collection<
    T extends SchemaType
> {
    static from<
        TS extends SchemaType = SchemaType
    >(name: string, schema: TS): Collection<TS>
    schema: T;
    constructor(name: string, schema: T);
    find(): Promise< Doc<T>[]>;
    findOne(id: string): Promise<Doc<T>>;
    update(id: string, document: Partial<Doc<T>>): Promise<void>;
    create(document: Doc<T>): Promise<Doc<T>>;
    delete(id: string): Promise<void>;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Deserialize, Serialize)]
pub struct Collection {
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) internals: Internals
}



#[wasm_bindgen]
impl Collection {

    #[wasm_bindgen(constructor)]
    pub fn from(name: String, internals: Internals) -> Collection {
        Collection {
            name,
            internals
        }
    }

    #[wasm_bindgen]
    pub async fn find(&self) -> Schema {
        todo!()
    }

    #[wasm_bindgen(js_name="findOne")]
    pub async fn find_one(&self) {
        todo!()
    }

    #[wasm_bindgen]
    pub async fn update(&self, document: JsValue) -> Result<JsValue, JsValue> {
        /* A document cannot get */
        todo!()
    }


    #[wasm_bindgen]
    pub async fn create(&self, document: JsValue) -> Result<JsValue, JsValue> {
        let result = self.internals.write(document).await;
        result.map_err(|e| JsValue::from(RIDBError::from(e)))
    }

    #[wasm_bindgen]
    pub async fn delete(&self) {
        todo!()
    }
}