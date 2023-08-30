use crate::parser::schema::shapes::Scalar;
use serde::{Deserialize, Serialize};

/// Interpolates the shape with its center point and bezier tangents with the opposite direction
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PuckerBloat {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,
    /// Amount as a percentage
    #[serde(rename = "a")]
    pub amount: Scalar,
}
