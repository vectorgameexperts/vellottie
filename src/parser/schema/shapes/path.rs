use serde::{Deserialize, Serialize};

use crate::parser::schema::animated_properties::shape_property::ShapeProperty;

use super::ShapeProperties;

/// Animatable Bezier curve
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PathShape {
    /// Base shape properties
    #[serde(flatten)]
    pub properties: ShapeProperties,
    /// Bezier path
    pub shape: ShapeProperty,
}
