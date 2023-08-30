use serde::{Deserialize, Serialize};

use crate::parser::schema::shapes::Shape;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]

/// Defines a character as shapes
pub struct CharacterShape {
    /// Shapes forming the character
    #[serde(rename = "shapes")]
    pub shapes: Vec<Shape>, // todo ShapeList?
}
