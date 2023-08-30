use serde::{Deserialize, Serialize};

use crate::parser::schema::constants::merge_mode::MergeMode;

/// Boolean operator on shapes

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MergeShape {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,
    /// Amount as a percentage
    #[serde(rename = "mm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_mode: Option<MergeMode>,
}
