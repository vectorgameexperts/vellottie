//! TextRangeSelector - https://lottiefiles.github.io/lottie-docs/text/#text-range-selector

use crate::parser::schema::animated_properties::AnimatedNumber;
use crate::parser::schema::helpers::int_boolean::BoolInt;

use serde::{Deserialize, Serialize};

/// Defines the range of characters to apply a property value only to a specific subset of the text document.
/// r Defines whether the values are defined as a percentage or indices.
/// The range is defined by s, e, and o.
/// ne and xe define what happes to text that is only partly inside the selected range.
/// b changes whether selection is done on per character basis, per word, etc. It also changes the meaning of an index when r is set to Indices.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TextRangeSelector {
    /// Expressible
    #[serde(rename = "t")]
    pub expressible: BoolInt,

    /// Max Ease
    #[serde(rename = "xe")]
    pub max_ease: AnimatedNumber,

    /// Min Ease
    #[serde(rename = "ne")]
    pub min_ease: AnimatedNumber,

    /// Max Amount
    #[serde(rename = "a")]
    pub max_amount: AnimatedNumber,

    /// Based On
    #[serde(rename = "b")]
    pub based_on: (), // todo text based

    /// Randomize
    #[serde(rename = "rn")]
    pub randomize: BoolInt,

    /// Shape
    #[serde(rename = "sh")]
    pub shape: (), // todo textShape

    /// Offset
    #[serde(rename = "o")]
    pub offset: AnimatedNumber,

    /// Range Units
    #[serde(rename = "r")]
    pub units: (), // todo textRangeUnits

    /// Selector Smoothness
    #[serde(rename = "sm")]
    pub smoothness: AnimatedNumber,

    /// Start
    #[serde(rename = "s")]
    pub start: AnimatedNumber,

    /// End
    #[serde(rename = "e")]
    pub end: AnimatedNumber,
}
