use crate::parser::schema::animated_properties::AnimatedVector;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]

/// Used to change the origin point for transformations, such as Rotation, that may be applied to the text string. The origin point for each character, word, or line can be changed.
pub struct TextAlignmentOptions {
    /// Group alignment
    #[serde(rename = "a")]
    pub group_alignment: (), // todo group alignment

    /// Anchor point grouping
    #[serde(rename = "g")]
    pub anchor_grouping: AnimatedVector, // todo amimated text document
}
