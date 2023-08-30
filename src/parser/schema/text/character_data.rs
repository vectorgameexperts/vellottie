use crate::parser::schema::text::character_precomp::CharacterPrecomp;
use crate::parser::schema::text::character_shapes::CharacterShape;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// You can also have font data directly into the lottie, this is done by having an array of character data objects in the chars attribute of the animation.

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CharacterData {
    /// Ranges
    #[serde(rename = "data")]
    pub data: ShapeOrPrecomp,

    /// Character
    #[serde(rename = "ch")]
    pub character: String,

    /// Font Family
    #[serde(rename = "fFamily")]
    pub font_family: String,

    /// Font Size
    #[serde(rename = "size")]
    pub size: Number,

    /// Font Style
    #[serde(rename = "style")]
    pub style: String,

    /// Width
    #[serde(rename = "w")]
    pub width: Number,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapeOrPrecomp {
    Shape(CharacterShape), // todo character shape
    Precomp(CharacterPrecomp),
}
