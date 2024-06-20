use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error::RDBError;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum OpType {
    CREATE,
    UPDATE,
    DELETE
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Operation<T extends SchemaType> = {
    collection: string,
    opType: OpType,
    data: Doc<T>,
    indexes: Array<string>
}
"#;

#[derive(Debug, Clone)]
#[wasm_bindgen(skip_typescript)]
pub struct Operation {
    pub(crate) collection: String,
    pub(crate) op_type: OpType,
    pub(crate) data: JsValue,
    pub(crate) indexes: Vec<String>
}

#[wasm_bindgen]
impl Operation {
    
    #[wasm_bindgen(getter)]
    pub fn collection(&self) -> String {
        self.collection.clone()
    }

    #[wasm_bindgen(getter, js_name="opType")]
    pub fn op_type(&self) -> OpType {
        self.op_type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        self.data.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn indexes(&self) -> Result<JsValue, JsValue> {
        to_value(&self.indexes.clone())
            .map_err(|e| JsValue::from(RDBError::from("Failed to retrieve value")))
    }
}