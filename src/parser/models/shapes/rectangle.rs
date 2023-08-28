use crate::parser::models::{
    animated_properties::AnimatedNumber,
    shapes::{AnimatedVector, ShapeProperties},
};
use serde::{Deserialize, Serialize};

/// A rectangle, defined by its center point and size.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RectangleShape {
    #[serde(flatten)]
    pub properties: ShapeProperties,
    /// Center of the rectangle
    #[serde(rename = "p")]
    pub position: AnimatedVector,
    /// Size
    #[serde(rename = "s")]
    pub size: AnimatedVector,
    /// Rounded corners radius
    #[serde(rename = "r")]
    pub rounded_corner_radius: AnimatedNumber,
}
