use crate::parser::schema::{
    animated_properties::color_value::ColorValue,
    constants::fill_rule::FillRule, shapes::FloatValue,
};
use serde::{Deserialize, Serialize};

use super::ShapeProperties;

/// Solid fill color
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FillShape {
    /// Shape Type
    #[serde(flatten)]
    pub properties: ShapeProperties,

    /// Opacity, 100 means fully opaque
    #[serde(rename = "o")]
    pub opacity: Option<FloatValue>,

    #[serde(rename = "c")]
    pub color: ColorValue,

    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<FillRule>,
}
