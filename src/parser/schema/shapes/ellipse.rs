use crate::parser::schema::{
    animated_properties::{
        multi_dimensional::MultiDimensional, position::Position,
    },
    shapes::ShapeProperties,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EllipseShape {
    #[serde(flatten)]
    pub properties: ShapeProperties,
    /// Position
    #[serde(rename = "p")]
    pub position: Position,
    /// Size
    #[serde(rename = "s")]
    pub size: MultiDimensional,
}
