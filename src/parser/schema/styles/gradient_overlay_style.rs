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
pub struct GradientOverlayStyle {
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
    #[serde(rename = "gf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<GradientColors>,
    ///
    #[serde(rename = "gs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothness: Option<FloatValue>,
    ///
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<FloatValue>,
    ///
    #[serde(rename = "gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_type: Option<GradientType>,
    ///
    #[serde(rename = "re")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<FloatValue>,
    /// Align with layer
    #[serde(rename = "al")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<FloatValue>,
    ///
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<FloatValue>,
    ///
    #[serde(rename = "of")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<FloatValue>,
}

impl GradientOverlayStyle {
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
        let angle = if let Ok(obj) = obj.extract_obj(breadcrumb, "a") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let offset = if let Ok(obj) = obj.extract_obj(breadcrumb, "d") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let scale = if let Ok(obj) = obj.extract_obj(breadcrumb, "s") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let gradient = if let Ok(obj) = obj.extract_obj(breadcrumb, "gf") {
            Some(GradientColors::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let gradient_type =
            obj.extract_type(breadcrumb, "gt", ValueType::EnumInt).ok();
        let smoothness = if let Ok(obj) = obj.extract_obj(breadcrumb, "gs") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let reverse = if let Ok(obj) = obj.extract_obj(breadcrumb, "re") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        let align = if let Ok(obj) = obj.extract_obj(breadcrumb, "al") {
            Some(FloatValue::from_obj(breadcrumb, &obj)?)
        } else {
            None
        };
        Ok(Self {
            layer_style,
            blend_mode,
            color,
            opacity,
            gradient,
            smoothness,
            angle,
            gradient_type,
            reverse,
            align,
            scale,
            offset,
        })
    }
}
