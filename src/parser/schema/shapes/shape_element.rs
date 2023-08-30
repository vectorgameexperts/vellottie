use crate::parser::schema::constants::blend_mode::BlendMode;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Base class for all elements of ShapeLayer and Group
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeElementShape {
    /// Whether the shape is hidden
    #[serde(rename = "hd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,

    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,

    /// Index used in expressions
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<Number>, //integer?

    /// CSS class used by the SVG renderer
    #[serde(rename = "cl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,

    /// `id` attribute used by the SVG renderer
    #[serde(rename = "ln")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,
}
