//! Text - https://lottiefiles.github.io/lottie-docs/layers/#text-layer

use super::common;
use crate::parser::schema::text::text_data::TextData;
use serde::{Deserialize, Serialize};
/// For text data, please refer to the section about text for details.
/// Also has the attributes from Visual Layer.
// TODO : Visual Layer
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TextLayer {
    #[serde(flatten)]
    pub properties: common::LayerProperties,
    /// Data
    #[serde(rename = "t")]
    pub text_data: TextData,
}
