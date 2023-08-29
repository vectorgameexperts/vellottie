use crate::parser::{breadcrumb::Breadcrumb, util::MapExt, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Asset {
    /// Unique identifier used by layers when referencing this asset
    #[serde(rename = "id")]
    pub id: String,
    /// Human readable name
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Asset {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let id = obj.extract_string(breadcrumb, "id")?;
        let name = obj.extract_string(breadcrumb, "nm").ok();
        Ok(Asset { id, name })
    }
}
