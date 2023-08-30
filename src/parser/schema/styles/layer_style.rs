use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::visual_object::VisualObject,
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Style Type
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum StyleType {
    Stroke = 0,
    DropShadow = 1,
    InnerShadow = 2,
    OuterGlow = 3,
    InnerGlow = 4,
    BevelEmboss = 5,
    Satin = 6,
    ColorOverlay = 7,
    GradientOverlay = 8,
}

/// Style applied to a layer
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayerStyle {
    #[serde(flatten)]
    pub visual_object: VisualObject,
    /// Style Type
    #[serde(rename = "ty")]
    pub style_type: StyleType,
}

impl LayerStyle {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let visual_object = VisualObject::from_obj(breadcrumb, obj);
        let style_type =
            obj.extract_type(breadcrumb, "ty", ValueType::EnumInt)?;
        Ok(LayerStyle {
            visual_object,
            style_type,
        })
    }
}
