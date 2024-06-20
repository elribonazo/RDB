use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use serde::de::value::Error;
use wasm_bindgen::prelude::*;
use crate::utils::extract_property;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Errors {
    Error,
    SerializationError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RDBError {
    pub code: Errors,
    pub message: String,
}

impl RDBError {
    pub fn error(err: String) -> RDBError {
        RDBError {
            code: Errors::Error,
            message: err
        }
    }
    pub fn serialisation(err: String) -> RDBError {
        RDBError {
            code: Errors::SerializationError,
            message: err
        }
    }
}

impl From<serde_wasm_bindgen::Error> for RDBError {
    fn from(error: serde_wasm_bindgen::Error) -> RDBError {
        RDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<JsValue> for RDBError {
    fn from(error: JsValue) -> RDBError {
        let code = extract_property::<Errors>(&error, "code").unwrap_or(Errors::Error);
        let message = extract_property::<String>(&error, "message").expect("Invalid JS Error no message is available");
        RDBError {
            code,
            message,
        }
    }
}

impl From<&str> for RDBError {
    fn from(error:&str) -> RDBError {
        RDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<String> for RDBError {
    fn from(error:String) -> RDBError {
        RDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<Error> for RDBError {
    fn from(error:Error) -> RDBError {
        RDBError {
            code: Errors::SerializationError,
            message:format!("Serialization {}", error),
        }
    }
}

impl From<RDBError> for JsValue {
    fn from(failure: RDBError) -> Self {
        let error = js_sys::Error::new(&failure.message).into();
        Reflect::set(&error, &"code".into(), &serde_wasm_bindgen::to_value(&failure.code).unwrap()).unwrap();
        error
    }
}

impl From<Errors> for JsValue {
    fn from(failure: Errors) -> Self {
        serde_wasm_bindgen::to_value(&failure).unwrap()
    }
}
