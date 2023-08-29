use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::keyframe::Keyframe;

/// Position Keyframe
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PositionKeyframe {
    #[serde(flatten)]
    pub keyframe: Keyframe,
    /// In-Tangent for values (e.g., moving position around a curved path).
    #[serde(rename = "ti")]
    pub value_in_tangent: Vec<Number>,
    /// Out-Tangent for values (e.g., moving position around a curved path).
    #[serde(rename = "to")]
    pub value_out_tangent: Vec<Number>,
}
