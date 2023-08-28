use crate::models::layer::AnimatedVector;
use serde::{Deserialize, Serialize};

/// Bezier path, note that it's a continuous shape, to have multiple shapes like when you need holes or gaps you need to create multiple Path shapes and group them together.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Path {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,
    /// Bezier path
    #[serde(rename = "ks")]
    pub bezier: AnimatedVector, // todo : animated bezier
}
