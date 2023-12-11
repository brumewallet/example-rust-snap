#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Context;

    #[wasm_bindgen(method)]
    fn request(this: &Context, request: &str) -> String;
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RpcRequest {
    pub(crate) method: String,
    pub(crate) params: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum RpcResponse<T> {
    Err(RpcErr),
    Ok(RpcOk<T>),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RpcOk<T> {
    pub(crate) result: T,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RpcErr {
    pub(crate) error: Value,
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

    fn request<T: DeserializeOwned>(&self, request: &Value) -> Result<T, JsError> {
        let json = self.context.request(&request.to_string());
        let value = serde_json::from_str::<RpcResponse<T>>(&json)?;

        match value {
            RpcResponse::Ok(ok) => Ok(ok.result),
            RpcResponse::Err(err) => Err(JsError::new(err.error.to_string().as_str())),
        }
    }

    #[wasm_bindgen]
    pub fn log(&self, text: &str) -> Result<(), JsError> {
        self.request::<Option<()>>(&json!({
          "method": "snap_log",
          "params": [text]
        }))?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn on_request(&self, request: &str) -> Result<String, JsError> {
        self.log(format!("on_request {}", request).as_str())?;

        let req = serde_json::from_str::<RpcRequest>(request)?;

        match req.method.as_str() {
            "echo" => Ok(self.on_echo(req)?.to_string()),
            _ => Err(JsError::new("Unknown method")),
        }
    }

    fn on_echo(&self, req: RpcRequest) -> Result<Value, JsError> {
        #[derive(Serialize, Deserialize)]
        pub(crate) struct Echo((String,));

        let Echo((message,)) = serde_json::from_str::<Echo>(&req.params.to_string())?;

        self.log(&message)?;

        Ok(json!(message))
    }
}
