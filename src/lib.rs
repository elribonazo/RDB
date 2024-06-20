
use serde::{Deserialize, Serialize};
use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

mod error;
mod utils;
mod schema;
mod collection;
mod storage;
mod database;
mod query;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}
