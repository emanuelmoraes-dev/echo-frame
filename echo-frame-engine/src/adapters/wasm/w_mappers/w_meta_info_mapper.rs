use crate::core::data::MetaInfoData;
use crate::adapters::wasm::w_data::WasmMetaInfo;

pub fn map(meta_info: &MetaInfoData) -> WasmMetaInfo {
    WasmMetaInfo {
        version: meta_info.version.into()
    }
}
