use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::int_boolean::BoolInt;

/// This represents a cubic bezier path.

/// Note that for interpolation to work correctly all bezier values in a property's keyframe must have the same number of points.

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Bezier {
    /// Whether the bezier forms a closed loop
    #[serde(rename = "c")]
    pub closed: BoolInt,

    /// Points along the curve
    #[serde(rename = "v")]
    pub vertices: Vec<[Number; 2]>,

    /// Cubic control points, incoming tangent
    #[serde(rename = "l")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_tangents: Option<Vec<[Number; 2]>>,

    /// Cubic control points, outgoing tangent
    #[serde(rename = "o")]
    pub out_tangents: Vec<[Number; 2]>, // todo 	2d vector
}
