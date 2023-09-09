use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::{
        animated_properties::{
            gradient_colors::GradientColors,
            multi_dimensional::MultiDimensional, value::FloatValue,
        },
        constants::gradient_type::GradientType,
    },
    util::MapExt,
    Error,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl Gradient {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let start_point = obj
            .extract_obj(breadcrumb, "s")
            .and_then(|obj| MultiDimensional::from_obj(breadcrumb, &obj))?;
        let end_point = obj
            .extract_obj(breadcrumb, "e")
            .and_then(|obj| MultiDimensional::from_obj(breadcrumb, &obj))?;
        let gradient_type =
            obj.extract_type(breadcrumb, "t", ValueType::EnumInt).ok();
        let highlight_length = obj
            .extract_obj(breadcrumb, "h")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();
        let highlight_angle = obj
            .extract_obj(breadcrumb, "a")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();
        let colors = obj
            .extract_obj(breadcrumb, "g")
            .and_then(|obj| GradientColors::from_obj(breadcrumb, &obj))?;

        Ok(Self {
            start_point,
            end_point,
            gradient_type,
            highlight_length,
            highlight_angle,
            colors,
        })
    }
}
