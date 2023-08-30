use crate::parser::schema::{
    animated_properties::{
        multi_dimensional::MultiDimensional, position::Position, value::Scalar,
    },
    shapes::ShapeProperties,
};
use serde::{Deserialize, Serialize};

/// A rectangle, defined by its center point and size.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RectangleShape {
    #[serde(flatten)]
    pub properties: ShapeProperties,
    /// Center of the rectangle
    #[serde(rename = "p")]
    pub position: Position,
    /// Size
    #[serde(rename = "s")]
    pub size: MultiDimensional,
    /// Rounded corners radius
    #[serde(rename = "r")]
    pub rounded_corner_radius: Scalar,
}
