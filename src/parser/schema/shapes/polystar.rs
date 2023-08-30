use crate::parser::schema::{
    animated_properties::{position::Position, value::Scalar},
    layers::enumerations::StarType,
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
    pub position: Position,
    /// Outer Radius
    #[serde(rename = "or")]
    pub outer_radius: Scalar,
    /// Outer Roundness as a percentage
    #[serde(rename = "os")]
    pub outer_roundness: Scalar,
    /// Rotation, clockwise in degrees
    #[serde(rename = "r")]
    pub rotation: Scalar,
    /// Points
    #[serde(rename = "pt")]
    pub points: Scalar,
    /// Star type, 1 for Star, 2 for Polygon
    #[serde(rename = "sy")]
    pub star_type: StarType,
    /// If sy is 1 (star) you also have attributes defining the inner ends of
    /// the star:
    /// Points
    #[serde(rename = "lr")]
    pub inner_radius: Option<Scalar>,
    /// Star type, 1 for Star, 2 for Polygon
    #[serde(rename = "ls")]
    pub inner_roundness: Option<Scalar>,
}
