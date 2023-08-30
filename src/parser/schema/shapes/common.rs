use super::ShapeType;
use crate::parser::{
    breadcrumb::Breadcrumb, breadcrumb::ValueType,
    schema::constants::blend_mode::BlendMode, util::MapExt, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Common properties between shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeProperties {
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "mn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_name: Option<String>,
    #[serde(rename = "ty")]
    pub shape_type: ShapeType,
    #[serde(rename = "hd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_in_expression: Option<Number>,
    #[serde(rename = "cl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,
    #[serde(rename = "ln")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ShapeProperties {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let name = obj.extract_string(breadcrumb, "nm").ok();
        let match_name = obj.extract_string(breadcrumb, "mn").ok();
        let shape_type: ShapeType =
            obj.extract_type(breadcrumb, "ty", ValueType::EnumInt)?;
        let hidden = obj.extract_bool(breadcrumb, "hd").ok();
        let blend_mode =
            obj.extract_type(breadcrumb, "bm", ValueType::EnumInt).ok();
        let index_in_expression = obj.extract_number(breadcrumb, "ix").ok();
        let css_class = obj.extract_string(breadcrumb, "cl").ok();
        let id = obj.extract_string(breadcrumb, "ln").ok();
        Ok(ShapeProperties {
            name,
            match_name,
            shape_type,
            hidden,
            blend_mode,
            index_in_expression,
            css_class,
            id,
        })
    }
}
