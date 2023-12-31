use super::shape_element::ShapeElement;
use super::stroke_dash::StrokeDash;
use crate::parser::schema::animated_properties::value::FloatValue;
use crate::parser::schema::constants::line_join::LineJoin;
use crate::parser::schema::{
    animated_properties::color_value::ColorValue, constants::line_cap::LineCap,
};
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Defines a stroke.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StrokeShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,
    /// Line Cap
    #[serde(rename = "lc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_cap: Option<LineCap>,
    /// Line Join
    #[serde(rename = "lj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_join: Option<LineJoin>,
    /// Miter Limit
    #[serde(rename = "ml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miter_limit: Option<Number>,
    /// Animatable alternative to miter_limit
    #[serde(rename = "ml2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miter_limit_alt: Option<FloatValue>,
    /// Opacity, 100 means fully opaque
    #[serde(rename = "o")]
    pub opacity: FloatValue,
    /// Stroke width
    #[serde(rename = "w")]
    pub stroke_width: FloatValue,
    /// Dashed line definition
    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_array: Option<Vec<StrokeDash>>,
    /// Stroke color
    #[serde(rename = "c")]
    pub stroke_color: ColorValue,
}
