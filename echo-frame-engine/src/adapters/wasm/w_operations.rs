use wasm_bindgen::prelude::*;

use super::w_data::WasmMetaInfo;
use super::w_mappers::w_meta_info_mapper;

use crate::core::ops;

#[wasm_bindgen(js_name = "getMetaInfo")]
pub fn w_get_meta_info() -> WasmMetaInfo {
    let meta_info_data = ops::get_meta_info();
    w_meta_info_mapper::map(&meta_info_data)
}
