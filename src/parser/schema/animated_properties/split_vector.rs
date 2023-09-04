use crate::parser::Error::UnexpectedChild;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    util::MapExt,
    Error,
};

use super::value::FloatValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An animatable property that is split into individually animated components.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SplitVector {
    /// Flag that is true for multidimensionals with individually animated components.
    #[serde(rename = "s")]
    pub split: bool,

    /// X component.
    #[serde(rename = "x")]
    pub x: FloatValue,

    /// Y component.
    #[serde(rename = "y")]
    pub y: FloatValue,

    /// Z component.
    #[serde(rename = "z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<FloatValue>,
}

impl SplitVector {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        obj.extract_bool(breadcrumb, "s").and_then(|split| {
            breadcrumb.enter_unnamed(ValueType::SplitVector);
            let result = if split {
                breadcrumb.enter(ValueType::SplitVector, Some("x"));
                let x = obj
                    .extract_obj(breadcrumb, "x")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?;
                breadcrumb.exit();
                breadcrumb.enter(ValueType::SplitVector, Some("y"));
                let y = obj
                    .extract_obj(breadcrumb, "y")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?;
                breadcrumb.exit();
                breadcrumb.enter(ValueType::SplitVector, Some("z"));
                let z = obj
                    .extract_obj(breadcrumb, "z")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
                    .ok();
                breadcrumb.exit();

                Ok(SplitVector { split, x, y, z })
            } else {
                Err(UnexpectedChild {
                    expected: ValueType::SplitVector,
                    breadcrumb: breadcrumb.clone(),
                })
            };
            breadcrumb.exit();
            result
        })
    }
}
