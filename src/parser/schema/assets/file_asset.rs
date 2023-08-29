use super::asset::Asset;
use crate::parser::{
    breadcrumb::Breadcrumb, schema::helpers::int_boolean::BoolInt,
    util::MapExt, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FileAsset {
    #[serde(flatten)]
    pub asset: Asset,
    /// Path to the directory containing a file
    #[serde(rename = "u")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    /// Filename or data url
    #[serde(rename = "p")]
    pub file_name: String,
    /// Whether the file is embedded
    #[serde(rename = "e", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<BoolInt>,
}

impl FileAsset {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let asset = Asset::from_obj(breadcrumb, obj)?;
        let dir = obj.extract_string(breadcrumb, "u").ok();
        let file_name = obj.extract_string(breadcrumb, "p")?;
        let embedded = obj.extract_bool_int(breadcrumb, "e").ok();
        Ok(FileAsset {
            asset,
            dir,
            file_name,
            embedded,
        })
    }
}
