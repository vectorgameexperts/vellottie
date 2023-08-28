//! Shapes - https://lottiefiles.github.io/lottie-docs/shapes/

use super::common;
use crate::parser::models::shapes::Shape;
use serde::{Deserialize, Serialize};

/// Has an array of shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeLayer {
    #[serde(flatten)]
    pub properties: common::LayerProperties,
    /// Has an array of shapes
    pub shapes: Vec<Shape>,
}
