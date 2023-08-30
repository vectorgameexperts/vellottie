use super::animated_property::AnimatedProperty;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::color::Color,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An animatable property that holds a Color.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ColorValue {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty<Color>,
}

impl ColorValue {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::ColorValue);
        let animated_property = AnimatedProperty::from_obj(breadcrumb, obj)?;
        let color = Self { animated_property };
        breadcrumb.exit();
        Ok(color)
    }
}
