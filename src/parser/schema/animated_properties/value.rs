use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::animated_property::{AnimatedProperty, AnimatedPropertyBase};

/// Animated Number
///
/// An animatable property that holds a float.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Animated(AnimatedProperty),
    Static(StaticValue),
}

/// Static value variant of a single float number.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StaticValue {
    #[serde(flatten)]
    animated_property_base: AnimatedPropertyBase,
    /// A single value.
    #[serde(rename = "k")]
    static_value: Number,
}
