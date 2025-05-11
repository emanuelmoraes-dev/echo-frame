use super::data::MetaInfoData;

pub fn get_meta_info() -> MetaInfoData {
    MetaInfoData {
        version: env!("CARGO_PKG_VERSION"),
    }
}
