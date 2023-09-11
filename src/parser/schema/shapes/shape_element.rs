use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::{
        constants::blend_mode::BlendMode, helpers::visual_object::VisualObject,
    },
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Base class for all elements of ShapeLayer and Group
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeElement {
    #[serde(flatten)]
    pub visual_object: VisualObject,

    /// Whether the shape is hidden
    #[serde(rename = "hd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,

    /// Index used in expressions
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<Number>,

    /// CSS class used by the SVG renderer
    #[serde(rename = "cl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,

    /// `id` attribute used by the SVG renderer
    #[serde(rename = "ln")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,

    /// TODO: This is an unknown property, but it showed up sometimes in test files.
    #[serde(rename = "ind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Number>,
}

impl ShapeElement {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let visual_object = VisualObject::from_obj(breadcrumb, obj);
        let hidden = obj.extract_bool(breadcrumb, "hd").ok();
        let blend_mode =
            obj.extract_type(breadcrumb, "bm", ValueType::EnumInt).ok();
        let property_index = obj.extract_number(breadcrumb, "ix").ok();
        let css_class = obj.extract_string(breadcrumb, "cl").ok();
        let xml_id = obj.extract_string(breadcrumb, "ln").ok();
        let index = obj.extract_number(breadcrumb, "ind").ok();
        Ok(ShapeElement {
            visual_object,
            hidden,
            blend_mode,
            property_index,
            css_class,
            xml_id,
            index,
        })
    }
}
