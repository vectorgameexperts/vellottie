use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::keyframe_base::KeyframeBase;

/// Keyframes specifies the value at a specific time and the interpolation function to reach the next keyframe.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Keyframe {
    #[serde(flatten)]
    base: KeyframeBase,
    /// Value at this keyframe. Note that if the property is a scalar, keyframe values are still represented as arrays.
    #[serde(rename = "s")]
    pub value: Vec<Number>,
    /// Value at the end of the keyframe. Note that this is deprecated, and you should use "s" from the next keyframe to get this value.
    #[serde(rename = "e")]
    #[deprecated(
        note = "you should use s from the next keyframe to get this value"
    )]
    pub end_value_deprecated: Option<Vec<Number>>,
}
