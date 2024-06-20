use js_sys::{Function};
use js_sys::JSON::{stringify_with_replacer, parse};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{ JsValue};

pub fn extract_property<T>(js_value: &JsValue, key: &str) -> Result<T, JsValue>
    where
        T: for<'de> serde::Deserialize<'de>,
{
    let prop = js_sys::Reflect::get(js_value, &JsValue::from(key)).expect("Error getting property");
    from_value(prop).map_err(|err| JsValue::from(err.to_string()))
}


pub fn fix_js_value(value: JsValue) -> JsValue {
    let replacer = Function::new_with_args(
        "key, value",
        "return value instanceof Map ? Object.fromEntries(value.entries()) : value;"
    );
    let json_string = stringify_with_replacer(&value, &replacer.into()).unwrap();
    let json_object = parse(json_string.as_string().unwrap().as_str()).unwrap();
    json_object
}
