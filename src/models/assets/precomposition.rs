use crate::{
    breadcrumb::Breadcrumb,
    models::{layer::Layer, BoolInt},
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Precomposition {
    /// Unique identifier used by layers when referencing this asset
    #[serde(rename = "id")]
    pub id: String,
    /// Human readable name
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Framerate in frames per second
    #[serde(rename = "fr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<Number>,
    /// Extra composition
    #[serde(rename = "xt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<BoolInt>,
    /// An array of layers
    #[serde(rename = "layers")]
    pub layers: Vec<Layer>,
}

impl Precomposition {
    pub fn from_object(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let id = obj.extract_string(breadcrumb, "id")?;
        let name = obj.extract_string(breadcrumb, "nm").ok();
        let frame_rate = obj.extract_number(breadcrumb, "fr").ok();
        let extra = obj.extract_bool_int(breadcrumb, "xt").ok();
        let mut layers = vec![];
        for v in obj.extract_arr(breadcrumb, "layers")? {
            breadcrumb.enter("layers".to_string());
            let layer = Layer::from_json(breadcrumb, &v)?;
            layers.push(layer);
            breadcrumb.exit();
        }
        Ok(Self {
            id,
            name,
            frame_rate,
            extra,
            layers,
        })
    }
}
