#![no_std]

extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Context;

    #[wasm_bindgen(method)]
    fn log(this: &Context, text: &str);
}

#[wasm_bindgen]
pub struct Snap {
    pub(crate) context: Context,
}

#[wasm_bindgen]
impl Snap {
    #[wasm_bindgen(constructor)]
    pub fn new(context: Context) -> Snap {
        Snap { context }
    }

    #[wasm_bindgen]
    pub fn on_request(&self, request: &str) -> Result<String, JsError> {
        self.context.log(format!("on_request {}", request).as_str());

        let req = serde_json::from_str::<Value>(request)?;

        let method = req
            .as_object()
            .unwrap_throw()
            .get("method")
            .unwrap_throw()
            .as_str()
            .unwrap_throw();

        match method {
            "hello" => Ok(self.on_hello(&req)?.to_string()),
            _ => Err(JsError::new("Unknown method")),
        }
    }

    fn on_hello(&self, req: &Value) -> Result<Value, JsError> {
        let params = req.get("params").unwrap_throw();

        let message = params
            .as_array()
            .unwrap_throw()
            .get(0)
            .unwrap_throw()
            .as_str()
            .unwrap_throw();

        match message {
            "world" => Ok(json!("Hello world")),
            _ => Err(JsError::new("Unknown message")),
        }
    }
}
