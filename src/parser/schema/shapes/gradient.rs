use crate::parser::schema::{
    animated_properties::{
        gradient_colors::GradientColors, multi_dimensional::MultiDimensional,
        value::FloatValue,
    },
    constants::gradient_type::GradientType,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
/// Represents a gradient.
pub struct Gradient {
    /// Describes the starting point for the gradient.
    #[serde(rename = "s")]
    pub start_point: MultiDimensional,

    /// Describes the end point for the gradient.
    #[serde(rename = "e")]
    pub end_point: MultiDimensional,

    /// Indicates the type of the gradient.
    #[serde(rename = "t")]
    pub gradient_type: Option<GradientType>,

    /// Represents the highlight length as a percentage between start and end points.
    #[serde(rename = "h")]
    pub highlight_length: Option<FloatValue>,

    /// Specifies the highlight angle relative to the direction from start to end points.
    #[serde(rename = "a")]
    pub highlight_angle: Option<FloatValue>,

    /// Describes the gradient colors.
    #[serde(rename = "g")]
    pub colors: GradientColors,
}