use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::schema::helpers::int_boolean::BoolInt;

use super::keyframe::Keyframe;

/// An animatable property that holds an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AnimatedPropertyBase {
    /// Property Index
    #[serde(flatten)]
    #[serde(rename = "ix")]
    pub property_index: Option<Number>,
    /// Whether the property is animated.
    #[serde(rename = "a")]
    pub animated: BoolInt,
    /// Expression for the property.
    #[serde(rename = "x")]
    pub expression: Option<String>,
    /// Slot ID
    #[serde(rename = "sid")]
    pub slot_id: Option<String>,
}

/// An animatable property that holds an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AnimatedProperty {
    #[serde(flatten)]
    pub animated_property: AnimatedPropertyBase,
    /// Animated value variant containing keyframes.
    #[serde(rename = "k")]
    pub animated_value: Vec<Keyframe>,
}
