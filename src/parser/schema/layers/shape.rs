//! Shapes - https://lottiefiles.github.io/lottie-docs/shapes/

use super::common::LayerProperties;
use crate::parser::schema::shapes::Shape;
use serde::{Deserialize, Serialize};

/// Has an array of shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeLayer {
    #[serde(flatten)]
    pub properties: LayerProperties,

    /// Has an array of shapes
    #[serde(rename = "shapes")]
    pub shapes: Vec<Shape>,
}
