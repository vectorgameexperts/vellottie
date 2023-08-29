use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::animated_property::{AnimatedProperty, AnimatedPropertyBase};

/// Animated Vector
///
/// An animatable property that holds an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MultiDimensional {
    Animated(AnimatedProperty),
    Static(StaticValue),
}

/// Static value variant of a float component array.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StaticValue {
    #[serde(flatten)]
    animated_property_base: AnimatedPropertyBase,
    /// A single component array.
    #[serde(rename = "k")]
    static_value: [Number; 2],
}
