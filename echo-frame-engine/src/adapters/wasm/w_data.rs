use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "MetaInfo")]
pub struct WasmMetaInfo {
    #[wasm_bindgen(getter_with_clone)]
    pub version: String
}
