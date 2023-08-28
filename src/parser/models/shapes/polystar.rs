use crate::parser::models::layer::{
    animated_properties::{AnimatedNumber, AnimatedVector},
    enumerations::StarType,
};
use serde::{Deserialize, Serialize};

/// Regular polygon or star.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PolyStar {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,
    /// Position
    #[serde(rename = "p")]
    pub position: AnimatedVector,
    /// Outer Radius
    #[serde(rename = "or")]
    pub outer_radius: AnimatedNumber,
    /// Outer Roundness as a percentage
    #[serde(rename = "os")]
    pub outer_roundness: AnimatedNumber,
    /// Rotation, clockwise in degrees
    #[serde(rename = "r")]
    pub rotation: AnimatedNumber,
    /// Points
    #[serde(rename = "pt")]
    pub points: AnimatedNumber,
    /// Star type, 1 for Star, 2 for Polygon
    #[serde(rename = "sy")]
    pub star_type: StarType,

    // todo:
    /// If sy is 1 (star) you also have attributes defining the inner ends of the star:

    /// Points
    #[serde(rename = "lr")]
    pub inner_radius: AnimatedNumber,
    /// Star type, 1 for Star, 2 for Polygon
    #[serde(rename = "ls")]
    pub inner_roundness: AnimatedNumber, // todo : StarType
}
