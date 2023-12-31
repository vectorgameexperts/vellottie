use super::{gradient::Gradient, shape_element::ShapeElement};
use crate::parser::schema::{
    animated_properties::value::FloatValue, constants::fill_rule::FillRule,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientFillShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    /// Gradient data
    #[serde(flatten)]
    pub gradient: Gradient,

    /// Opacity
    #[serde(rename = "o")]
    pub opacity: FloatValue,

    /// Fill Rule
    #[serde(rename = "r")]
    pub fill_rule: Option<FillRule>,
}
