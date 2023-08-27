use super::{animated_properties::AnimatedNumber, common};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrecompositionLayer {
    #[serde(flatten)]
    pub properties: common::LayerProperties,
    /// ID of the precomp as specified in the assets
    #[serde(rename = "refID")]
    pub precomp_id: String,
    /// Width of the clipping rect
    #[serde(rename = "w")]
    pub width: Number,
    /// Height of the clipping rect
    #[serde(rename = "h")]
    pub height: Number,
    /// Time Remapping
    #[serde(rename = "tm")]
    pub time_remap: AnimatedNumber,
}
