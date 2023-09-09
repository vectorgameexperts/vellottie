use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::{
        animated_properties::value::FloatValue,
        constants::stroke_dash_type::StrokeDashType,
        helpers::visual_object::VisualObject,
    },
    util::MapExt,
    Error,
};

/// An item used to described the dashe pattern in a stroked path
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StrokeDash {
    #[serde(flatten)]
    visual_object: VisualObject,

    #[serde(rename = "n")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dash_type: Option<StrokeDashType>,

    /// Length of the dash
    #[serde(rename = "v")]
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<FloatValue>,
}

impl StrokeDash {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let visual_object = VisualObject::from_obj(breadcrumb, obj);
        let dash_type =
            obj.extract_type(breadcrumb, "n", ValueType::EnumInt).ok();
        let length = obj
            .extract_obj(breadcrumb, "v")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();
        Ok(Self {
            visual_object,
            dash_type,
            length,
        })
    }
}
