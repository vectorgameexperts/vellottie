use super::layer_style::LayerStyle;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::{
        animated_properties::{
            color_value::ColorValue, gradient_colors::GradientColors,
            value::FloatValue,
        },
        constants::gradient_type::GradientType,
    },
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ColorOverlayStyle {
    #[serde(flatten)]
    pub layer_style: LayerStyle,
    ///
    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<FloatValue>,
    ///
    #[serde(rename = "c")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorValue>,
    ///
    #[serde(rename = "so")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,
}

impl ColorOverlayStyle {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let layer_style = LayerStyle::from_obj(breadcrumb, obj)?;
        let blend_mode = if let Ok(obj) = obj.extract_obj(breadcrumb, "bm") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let color = if let Ok(obj) = obj.extract_obj(breadcrumb, "c") {
            Some(ColorValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let opacity = if let Ok(obj) = obj.extract_obj(breadcrumb, "so") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        Ok(Self {
            layer_style,
            blend_mode,
            color,
            opacity,
        })
    }
}
