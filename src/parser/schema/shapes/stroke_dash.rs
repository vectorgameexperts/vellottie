use serde::{Deserialize, Serialize};

use crate::parser::schema::{
    animated_properties::value::FloatValue,
    constants::stroke_dash_type::StrokeDashType,
    helpers::visual_object::VisualObject,
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
