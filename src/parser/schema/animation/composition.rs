use crate::parser::{
    breadcrumb::Breadcrumb, breadcrumb::ValueType, schema::layers::AnyLayer,
    util::MapExt, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Base class for layer holders
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Composition {
    /// An array of Layers
    #[serde(rename = "layers")]
    pub layers: Vec<AnyLayer>,
}

impl Composition {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let mut layers = vec![];
        let json_layers = obj.extract_arr(breadcrumb, "layers")?;
        breadcrumb.enter(ValueType::Array, Some("layers"));
        for v in json_layers {
            let layer = AnyLayer::from_json(breadcrumb, &v)?;
            layers.push(layer);
        }
        breadcrumb.exit();

        Ok(Self { layers })
    }
}
