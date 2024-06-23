
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
mod error;
mod utils;
mod schema;
mod collection;
mod storage;
mod database;
mod query;

#[cfg(any(feature = "browser", feature = "node"))]
mod tests_specific {
    use wasm_bindgen_test::{wasm_bindgen_test_configure};

    pub fn configure() {
        #[cfg(feature = "browser")]
        wasm_bindgen_test_configure!(run_in_browser);
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
