use super::animated_property::AnimatedProperty;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// aka Value, in the Schema.
///
/// An animatable property that holds a float.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FloatValue {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty<Number>,
}

impl FloatValue {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::Value);
        let animated_property = AnimatedProperty::from_obj(breadcrumb, obj)?;
        let value = Self { animated_property };
        breadcrumb.exit();
        Ok(value)
    }
}
