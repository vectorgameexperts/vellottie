use super::keyframe_bezier_handle::KeyframeBezierHandle;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::int_boolean::BoolInt,
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// A Keyframes specifies the value at a specific time and the interpolation function to reach the next keyframe.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyframeBase {
    /// Time
    #[serde(rename = "t")]
    pub time: Number,
    /// Hold
    #[serde(rename = "h")]
    pub hold: BoolInt,
    /// In tangent of the keyframe.
    /// Easing tangent going into the next keyframe.
    #[serde(rename = "i")]
    pub in_tangent: KeyframeBezierHandle,
    /// Out tangent of the keyframe.
    /// Easing tangent leaving the current keyframe.
    #[serde(rename = "o")]
    pub out_tangent: KeyframeBezierHandle,
}

impl KeyframeBase {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::ShapeProperty);
        let time = obj.extract_number(breadcrumb, "t")?;
        let hold = obj.extract_bool_int(breadcrumb, "h")?;
        let in_tangent = KeyframeBezierHandle::from_obj(
            breadcrumb,
            &obj.extract_obj(breadcrumb, "i")?,
        )?;
        let out_tangent = KeyframeBezierHandle::from_obj(
            breadcrumb,
            &obj.extract_obj(breadcrumb, "o")?,
        )?;
        Ok(Self {
            time,
            hold,
            in_tangent,
            out_tangent,
        })
    }
}
