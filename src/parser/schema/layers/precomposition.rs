use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::schema::animated_properties::value::Scalar;

use super::common::LayerProperties;

/// Renders a Precomposition
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrecompositionLayer {
    #[serde(flatten)]
    pub properties: LayerProperties,
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
    pub time_remap: Scalar,
}
