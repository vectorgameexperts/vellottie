use crate::parser::models::{
    layer::transform::Transform, shapes::ShapeProperties,
};
use serde::{Deserialize, Serialize};

/// A rectangle, defined by its center point and size.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TransformShape {
    #[serde(flatten)]
    pub properties: ShapeProperties,
    #[serde(flatten)]
    pub transform: Transform,
}
