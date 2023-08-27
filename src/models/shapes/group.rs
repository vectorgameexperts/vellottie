use super::{common, Shape};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GroupShape {
    #[serde(flatten)]
    pub properties: common::ShapeProperties,
    /// Number of properties
    #[serde(rename = "np")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_properties: Option<Number>,
    /// Array of shapes
    #[serde(rename = "it")]
    pub shapes: Vec<Shape>,
}
