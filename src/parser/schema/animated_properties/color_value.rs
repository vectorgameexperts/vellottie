use super::{animated_property::AnimatedProperty, keyframe::Keyframe};
use crate::parser::schema::helpers::{color::Color, int_boolean::BoolInt};
use serde::{Deserialize, Serialize};

/// An animatable property that holds a Color.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ColorValue {
    #[serde(flatten)]
    pub animated_property: Option<AnimatedProperty>,
    #[serde(rename = "k")]
    pub value: ColorValueK,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum ColorValueK {
    /// Keyframes specifies the value at a specific time and the interpolation function to reach the next keyframe.
    Animated(Vec<Keyframe>),
    /// Static value
    Static(Color),
}
