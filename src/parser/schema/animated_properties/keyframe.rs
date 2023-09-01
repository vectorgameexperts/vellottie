use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    util::MapExt,
    Error,
};

use super::keyframe_base::KeyframeBase;

/// Keyframes specifies the value at a specific time and the interpolation function to reach the next keyframe.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Keyframe {
    #[serde(flatten)]
    pub base: KeyframeBase,
    /// Value at this keyframe. Note that if the property is a scalar, keyframe values are still represented as arrays.
    #[serde(rename = "s")]
    pub value: Vec<Number>,
    /// Value at the end of the keyframe. Note that this is deprecated, and you should use "s" from the next keyframe to get this value.
    #[serde(rename = "e")]
    #[deprecated(
        note = "you should use s from the next keyframe to get this value"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value_deprecated: Option<Vec<Number>>,
}

#[allow(deprecated)]
impl Keyframe {
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
        breadcrumb.exit();
        Ok(keyframe)
    }
}
