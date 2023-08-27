use crate::models::layer::animated_properties::AnimatedVector;
use crate::models::shapes::ShapeProperties;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EllipseShape {
    #[serde(flatten)]
    pub properties: ShapeProperties,
    /// Position
    #[serde(rename = "p")]
    pub position: AnimatedVector,
    /// Size
    #[serde(rename = "s")]
    pub size: AnimatedVector,
}
