use crate::parser::schema::{
    animated_properties::value::FloatValue, constants::fill_rule::FillRule,
};

use super::{gradient::GradientShape, ShapeProperties};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientFillShape {
    /// Common properties between shapes
    #[serde(flatten)]
    pub properties: ShapeProperties,

    /// Gradient data
    #[serde(flatten)]
    pub shape: GradientShape,

    /// Opacity
    #[serde(rename = "o")]
    pub opacity: FloatValue,

    /// Fill Rule
    #[serde(rename = "r")]
    pub fill_rule: Option<FillRule>,
}
