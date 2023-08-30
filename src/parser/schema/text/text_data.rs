use serde::{Deserialize, Serialize};

/// This object is similar to an animated property for text.

/// The main difference is that it's always treated as animated (ie: you must use keyframes).
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TextData {
    /// Ranges
    #[serde(rename = "a")]
    pub text_range_array: (), // todo text range array

    /// Document
    #[serde(rename = "d")]
    pub document: (), // todo amimated text document

    /// Alignment
    #[serde(rename = "m")]
    pub text_alignment_options: (), // todo text alignment options

    /// Follow Path
    #[serde(rename = "p")]
    pub text_follow_path: (), // todo text follow path
}
