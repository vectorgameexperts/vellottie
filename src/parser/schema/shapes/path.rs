use serde::{Deserialize, Serialize};

use crate::parser::schema::animated_properties::shape_property::ShapeProperty;

use super::shape_element::ShapeElement;

/// Animatable Bezier curve
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PathShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,
    /// Bezier path
    #[serde(flatten)]
    pub shape: ShapeProperty,
}
