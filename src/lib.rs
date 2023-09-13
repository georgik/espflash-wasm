extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn flash_device(config: JsValue) -> Result<JsValue, JsValue> {
    Err(JsValue::from_str("Not implemented"))
}