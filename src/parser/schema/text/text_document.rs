use crate::parser::schema::layers::enumerations::TextCaps; //todo move from layers to text
use crate::parser::schema::layers::enumerations::TextJustify; // todo move from layers to text

use serde::{Deserialize, Serialize};
use serde_json::Number;

/// This is where the actual text data is stored.

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TextDocument {
    /// Font Family
    #[serde(rename = "f")]
    pub font_family: String,

    /// Fill Color
    #[serde(rename = "fc")]
    pub fill_color: (), // todo Color

    /// Stroke Color
    #[serde(rename = "sc")]
    pub stroke_color: (), // todo Color

    /// Stroke Width
    #[serde(rename = "sw")]
    pub stroke_width: Number,

    /// Render stroke above the fill
    #[serde(rename = "of")]
    pub above_fill: bool,

    /// Font Size
    #[serde(rename = "s")]
    pub font_size: Number,

    /// Distance between lines on multiline or wrapped text
    #[serde(rename = "lh")]
    pub line_height: Number,

    /// Size of the box containing the text
    #[serde(rename = "sz")]
    pub box_size: Vec<Number>,

    /// Position of the box containing the text
    #[serde(rename = "ps")]
    pub box_position: Vec<Number>,

    /// Text, note that newlines are encoded with \r
    #[serde(rename = "t")]
    pub text: String,

    /// Text Justify
    #[serde(rename = "j")]
    pub text_justify: TextJustify,

    /// Text Caps
    #[serde(rename = "ca")]
    pub text_caps: TextCaps,

    /// Text Tracking
    #[serde(rename = "tr")]
    pub text_tracking: Number,

    /// baseline shift
    #[serde(rename = "ls")]
    pub baseline_shift: Number,
}
