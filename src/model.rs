use crate::util::{self};
use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Lottie {
    /// Lottie file version
    #[serde(rename = "v")]
    pub version: String,
    /// Framerate in frames per second
    #[serde(rename = "fr")]
    pub frame_rate: f64,
    /// "In Point", which frame the animation starts at (usually 0)
    #[serde(rename = "ip")]
    pub in_point: f64,
    /// "Out Point", which frame the animation stops/loops at, which makes this the duration in frames when `ip` is 0
    #[serde(rename = "op")]
    pub out_point: f64,
    /// Width of the animation
    #[serde(rename = "w")]
    pub width: i64,
    /// Height of the animation
    #[serde(rename = "h")]
    pub height: i64,
    /// Whether the animation has 3D layers
    #[serde(
        rename = "ddd",
        deserialize_with = "util::bool_from_int",
        serialize_with = "util::bool_to_int",
        default
    )]
    pub three_dimensional: bool,
}

impl Lottie {
    pub fn to_json(&self) -> serde_json::value::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Display for Lottie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
