use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::{
        animated_properties::{
            shape_property::ShapeProperty, value::FloatValue,
        },
        constants::mask_mode::MaskMode,
    },
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};

/// A layer can have an array of masks that clip the contents of the layer to a shape.

/// This is similar to mattes, but there are a few differences.

/// With mattes, you use a layer to define the clipping area, while with masks you use an animated bezier curve.

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Mask {
    /// Name, as seen from editors and the like
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Match name, used in expressions
    #[serde(rename = "mn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_name: Option<String>,

    /// Inverted
    #[serde(rename = "inv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,

    /// Shape
    #[serde(rename = "pt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<ShapeProperty>,

    /// Opacity
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,

    /// Mode
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<MaskMode>,

    /// Expand
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<FloatValue>,
}

impl Mask {
    pub fn from_json(
        breadcrumb: &mut Breadcrumb,
        v: &serde_json::Value,
    ) -> Result<Self, Error> {
        let obj = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Mask,
        })?;

        let name = obj.extract_string(breadcrumb, "nm").ok();
        let match_name = obj.extract_string(breadcrumb, "mn").ok();
        let inverted = obj.extract_bool(breadcrumb, "inv").ok();
        let shape = obj
            .extract_obj(breadcrumb, "pt")
            .and_then(|obj| ShapeProperty::from_obj(breadcrumb, &obj))
            .ok();
        let opacity = obj
            .extract_obj(breadcrumb, "o")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();
        let mode: Option<MaskMode> = obj
            .extract_type(
                breadcrumb,
                "mode",
                crate::parser::breadcrumb::ValueType::EnumStr,
            )
            .ok();
        let expand = obj
            .extract_obj(breadcrumb, "x")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();

        Ok(Mask {
            name,
            match_name,
            inverted,
            shape,
            opacity,
            mode,
            expand,
        })
    }
}
