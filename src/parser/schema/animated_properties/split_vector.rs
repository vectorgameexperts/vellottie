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
    pub z: Option<FloatValue>,
}

impl SplitVector {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::SplitVector);
        let vector = obj.extract_bool(breadcrumb, "s").and_then(|split| {
            if split {
                Ok(SplitVector {
                    split,
                    x: FloatValue::from_obj(breadcrumb, obj)?,
                    y: FloatValue::from_obj(breadcrumb, obj)?,
                    z: FloatValue::from_obj(breadcrumb, obj).ok(),
                })
            } else {
                Err(UnexpectedChild {
                    expected: ValueType::SplitVector,
                    breadcrumb: breadcrumb.clone(),
                })
            }
        });
        breadcrumb.exit();
        vector
    }
}
