use super::animated_property::AnimatedProperty;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// An animatable property that holds an array of numbers
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MultiDimensional {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty<[Number; 2]>,
}

impl MultiDimensional {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::MultiDimensional);
        let animated_property = AnimatedProperty::from_obj(breadcrumb, obj)?;
        let multi_dimensional = Self { animated_property };
        breadcrumb.exit();
        Ok(multi_dimensional)
    }
}
