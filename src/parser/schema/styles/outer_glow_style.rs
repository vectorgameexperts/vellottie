use super::layer_style::LayerStyle;
use crate::parser::{
    breadcrumb::Breadcrumb,
    schema::animated_properties::{color_value::ColorValue, value::FloatValue},
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OuterGlowStyle {
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
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,
    ///
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<FloatValue>,
    ///
    #[serde(rename = "ch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choke_spread: Option<FloatValue>,
    ///
    #[serde(rename = "no")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise: Option<FloatValue>,
    ///
    #[serde(rename = "j")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter: Option<FloatValue>,
}

impl OuterGlowStyle {
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
        let opacity = if let Ok(obj) = obj.extract_obj(breadcrumb, "o") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let choke_spread = if let Ok(obj) = obj.extract_obj(breadcrumb, "ch") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let range = if let Ok(obj) = obj.extract_obj(breadcrumb, "r") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let noise = if let Ok(obj) = obj.extract_obj(breadcrumb, "no") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let jitter = if let Ok(obj) = obj.extract_obj(breadcrumb, "j") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        Ok(Self {
            layer_style,
            blend_mode,
            color,
            opacity,
            choke_spread,
            range,
            noise,
            jitter,
        })
    }
}
