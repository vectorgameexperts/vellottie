use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    util::MapExt,
    Error,
};

use super::{keyframe::Keyframe, keyframe_base::KeyframeBase};
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Position Keyframe
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PositionKeyframe {
    #[serde(flatten)]
    pub keyframe: Keyframe,
    /// In-Tangent for values (e.g., moving position around a curved path).
    #[serde(rename = "ti")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_tangent: Option<Vec<Number>>,
    /// Out-Tangent for values (e.g., moving position around a curved path).
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_out_tangent: Option<Vec<Number>>,
}

#[allow(deprecated)]
impl PositionKeyframe {
    pub fn from_json(
        breadcrumb: &mut Breadcrumb,
        v: &serde_json::Value,
    ) -> Result<Self, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Keyframe,
        })?;
        breadcrumb.enter_unnamed(ValueType::Keyframe);

        let base = KeyframeBase::from_obj(breadcrumb, root)?;
        let value = root.extract_type(breadcrumb, "s", ValueType::Array)?;
        let end_value_deprecated =
            root.extract_type(breadcrumb, "e", ValueType::Array).ok();

        let keyframe = Keyframe {
            base,
            value,
            end_value_deprecated,
        };

        let value_in_tangent =
            root.extract_type(breadcrumb, "ti", ValueType::Array).ok();

        let value_out_tangent =
            root.extract_type(breadcrumb, "to", ValueType::Array).ok();

        breadcrumb.exit();

        Ok(PositionKeyframe {
            keyframe,
            value_in_tangent,
            value_out_tangent,
        })
    }
}
