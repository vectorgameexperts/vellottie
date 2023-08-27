use super::{common, Shape};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GroupShape {
    #[serde(flatten)]
    properties: common::ShapeProperties,
    /// Number of properties
    #[serde(rename = "np")]
    #[serde(skip_serializing_if = "Option::is_none")]
    num_properties: Option<Number>,
    /// Array of shapes
    #[serde(rename = "it")]
    #[serde(skip_serializing_if = "Option::is_none")]
    shapes: Option<Vec<Shape>>,
}
