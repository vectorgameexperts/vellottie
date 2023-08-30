use super::layer_style::LayerStyle;
use crate::parser::{
    breadcrumb::Breadcrumb,
    schema::animated_properties::{color_value::ColorValue, value::FloatValue},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Style applied to a layer
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SatinStyle {
    #[serde(flatten)]
    pub layer_style: LayerStyle,
    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<FloatValue>,
    #[serde(rename = "c")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorValue>,
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<FloatValue>,
    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<FloatValue>,
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<FloatValue>,
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<FloatValue>,
}

impl SatinStyle {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let layer_style = LayerStyle::from_obj(breadcrumb, obj)?;
        let blend_mode = FloatValue::from_obj(breadcrumb, obj).ok();
        let color = ColorValue::from_obj(breadcrumb, obj).ok();
        let opacity = FloatValue::from_obj(breadcrumb, obj).ok();
        let angle = FloatValue::from_obj(breadcrumb, obj).ok();
        let distance = FloatValue::from_obj(breadcrumb, obj).ok();
        let size = FloatValue::from_obj(breadcrumb, obj).ok();
        let invert = FloatValue::from_obj(breadcrumb, obj).ok();
        Ok(Self {
            layer_style,
            blend_mode,
            color,
            opacity,
            angle,
            distance,
            size,
            invert,
        })
    }
}
