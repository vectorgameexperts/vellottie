use crate::parser::{
    breadcrumb::Breadcrumb, schema::helpers::visual_object::VisualObject,
    util::MapExt, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Style applied to a layer
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayerStyle {
    #[serde(flatten)]
    pub visual_object: VisualObject,
    /// Style Type
    #[serde(rename = "ty")]
    pub style_type: Number, // todo - this should be an enum but idk the valid range of integers
}

impl LayerStyle {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let visual_object = VisualObject::from_obj(breadcrumb, obj);
        let style_type = obj.extract_number(breadcrumb, "ty")?;
        Ok(LayerStyle {
            visual_object,
            style_type,
        })
    }
}
