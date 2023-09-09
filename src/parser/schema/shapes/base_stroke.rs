use super::stroke_dash::StrokeDash;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::{
        animated_properties::value::FloatValue,
        constants::{line_cap::LineCap, line_join::LineJoin},
    },
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BaseStroke {
    /// Line Cap
    #[serde(rename = "lc")]
    pub line_cap: Option<LineCap>,

    /// Line Join
    #[serde(rename = "lj")]
    pub line_join: Option<LineJoin>,

    /// Miter Limit
    #[serde(rename = "ml")]
    pub miter_limit: Option<Number>,

    /// Animatable alternative to miter limit
    #[serde(rename = "ml2")]
    pub miter_limit_anim: Option<FloatValue>,

    /// Opacity
    #[serde(rename = "o")]
    pub opacity: FloatValue,

    /// Stroke Width
    #[serde(rename = "w")]
    pub width: FloatValue,

    /// Dashed line definition
    #[serde(rename = "d")]
    pub dashes: Option<Vec<StrokeDash>>,
}

impl BaseStroke {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let line_cap =
            obj.extract_type(breadcrumb, "lc", ValueType::EnumInt).ok();
        let line_join =
            obj.extract_type(breadcrumb, "lj", ValueType::EnumInt).ok();
        let miter_limit = obj.extract_number(breadcrumb, "ml").ok();
        let miter_limit_anim = obj
            .extract_type(breadcrumb, "ml2", ValueType::Scalar2d)
            .ok();
        let opacity = obj
            .extract_obj(breadcrumb, "o")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?;
        let width = obj
            .extract_obj(breadcrumb, "w")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?;
        let dashes = obj.extract_type(breadcrumb, "d", ValueType::EnumStr).ok();

        Ok(Self {
            line_cap,
            line_join,
            miter_limit,
            miter_limit_anim,
            opacity,
            width,
            dashes,
        })
    }
}
