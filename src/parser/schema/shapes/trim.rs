use crate::parser::schema::{
    animated_properties::value::FloatValue,
    constants::trim_multiple_shapes::TrimMultipleShapes,
};

use super::ShapeProperties;
use serde::{Deserialize, Serialize};

/// Trims shapes into a segment.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TrimShape {
    /// Shape Type
    #[serde(flatten)]
    pub properties: ShapeProperties,

    /// Segment start.
    #[serde(rename = "s")]
    pub start: FloatValue,

    /// Segment end.
    #[serde(rename = "e")]
    pub end: FloatValue,

    /// Offset.
    #[serde(rename = "o")]
    pub offset: FloatValue,

    /// How to treat multiple copies.
    #[serde(rename = "m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<TrimMultipleShapes>,
}
