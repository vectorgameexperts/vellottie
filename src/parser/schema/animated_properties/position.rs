use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::schema::helpers::int_boolean::BoolInt;

use super::position_keyframe::PositionKeyframe;

/// An animatable property to represent a position in space
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AnimatedPosition {
    /// The index of the property.
    #[serde(rename = "ix")]
    pub property_index: Option<Number>,
    /// Whether the property is animated
    #[serde(rename = "a")]
    pub animated: Option<BoolInt>,
    /// The expression for the property.
    #[serde(rename = "x")]
    pub expression: Option<String>,
    /// Number of components in the value arrays.
    /// If present, values will be truncated or expanded to match this length when accessed from expressions.
    #[serde(rename = "l")]
    pub length: Option<Number>,
    /// The value variant (Animated or Static).
    #[serde(flatten)]
    pub value: PositionValue,
}

/// Static value variant containing an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StaticValue {
    /// Array of static values.
    #[serde(rename = "k")]
    static_value: Vec<Number>,
}

/// Animated value variant containing keyframes.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AnimatedValue {
    /// Array of keyframes.
    #[serde(rename = "k")]
    animated_value: Vec<PositionKeyframe>,
}

/// Represents the two possible value variants for the animated position property.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PositionValue {
    Static(StaticValue),
    Animated(AnimatedValue),
}
