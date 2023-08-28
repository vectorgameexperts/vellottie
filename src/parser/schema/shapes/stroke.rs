use crate::parser::schema::animated_properties::AnimatedNumber;
use crate::parser::schema::animated_properties::AnimatedVector;
use crate::parser::schema::shapes::ShapeProperties;
use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::schema::shapes::enumerations::{
    LineCap, LineJoin, StrokeDash,
};

/// Defines a stroke.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StrokeShape {
    /// Shape Type
    #[serde(flatten)]
    pub properties: ShapeProperties,
    /// Line Cap
    #[serde(rename = "lc")]
    pub line_cap: Option<LineCap>,
    /// Line Join
    #[serde(rename = "lj")]
    pub line_join: Option<LineJoin>,
    /// Miter Limit
    #[serde(rename = "ml")]
    pub miter_limit: Option<Number>,
    /// Animatable alternative to miter_limit
    #[serde(rename = "ml2")]
    pub miter_limit_alt: Option<AnimatedNumber>,
    /// Opacity, 100 means fully opaque
    #[serde(rename = "o")]
    pub opacity: AnimatedNumber,
    /// Stroke width
    #[serde(rename = "w")]
    pub stroke_width: AnimatedNumber,
    /// Dashed line definition
    #[serde(rename = "d")]
    pub dash_array: Option<Vec<StrokeDash>>,
    /// Stroke color
    #[serde(rename = "c")]
    pub stroke_color: AnimatedVector,
}
