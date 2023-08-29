use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::schema::helpers::int_boolean::BoolInt;

use super::keyframe_bezier_handle::KeyframeBezierHandle;

/// An animatable property to represent a position in space
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyframeBase {
    /// Time
    #[serde(rename = "t")]
    pub time: Number,
    /// Hold
    #[serde(rename = "h")]
    pub hold: BoolInt,
    /// In tangent of the keyframe.
    /// Easing tangent going into the next keyframe.
    #[serde(rename = "i")]
    pub in_tangent: KeyframeBezierHandle,
    /// Out tangent of the keyframe.
    /// Easing tangent leaving the current keyframe.
    #[serde(rename = "o")]
    pub out_tangent: KeyframeBezierHandle,
}
