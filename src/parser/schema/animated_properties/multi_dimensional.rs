use super::animated_property::AnimatedProperty;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// An animatable property that holds an array of numbers
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MultiDimensional {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty<Vec<Number>>,

    /// Number of components in the value arrays.
    /// If present values will be truncated or expanded to match this length when accessed from expressions.
    #[serde(rename = "l")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Number>,
}

impl MultiDimensional {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::MultiDimensional);
        let animated_property = AnimatedProperty::from_obj(breadcrumb, obj)?;
        let length = obj.extract_number(breadcrumb, "l").ok();
        let multi_dimensional = Self {
            animated_property,
            length,
        };
        breadcrumb.exit();
        Ok(multi_dimensional)
    }
}
