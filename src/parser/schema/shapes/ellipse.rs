use super::shape_element::ShapeElement;
use crate::parser::schema::animated_properties::{
    multi_dimensional::MultiDimensional, position::Position,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EllipseShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,
    /// Position
    #[serde(rename = "p")]
    pub position: Position,
    /// Size
    #[serde(rename = "s")]
    pub size: MultiDimensional,
}
