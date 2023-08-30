use crate::parser::schema::transform::Transform;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Defines a character as a precomp layer

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CharacterPrecomp {
    /// Ranges
    #[serde(rename = "refId")]
    pub precomp_id: String,

    /// Layer transform
    #[serde(rename = "ks")]
    pub layer_transform: Transform,

    /// Frame when the layer becomes visible
    #[serde(rename = "ip")]
    pub frame_visible: Number,

    /// Frame when the layer becomes invisible
    #[serde(rename = "op")]
    pub frame_invisible: Number,

    /// Time Stretch
    #[serde(rename = "sr")]
    pub time_stretch: Number,

    /// Start Time
    #[serde(rename = "st")]
    pub start_time: Number,
}
