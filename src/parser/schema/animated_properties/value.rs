use super::{animated_property::AnimatedProperty, keyframe::Keyframe};
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::int_boolean::BoolInt,
    util::MapExt,
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
    pub animated_property: AnimatedProperty,
    /// A single value.
    #[serde(rename = "k")]
    pub value: FloatValueK,
}

/// The possible values of "k" in [`FloatValue`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum FloatValueK {
    /// Keyframes specifies the value at a specific time and the interpolation function to reach the next keyframe.
    Animated(Vec<Keyframe>),
    /// Static value
    Static(Number),
}

impl FloatValue {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::AnimatedNumber);

        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        let number = if animated == BoolInt::True {
            todo!();
        } else {
            FloatValue {
                animated_property: AnimatedProperty {
                    property_index: obj.extract_number(breadcrumb, "ix").ok(),
                    animated,
                    expression: obj.extract_string(breadcrumb, "x").ok(),
                    slot_id: obj.extract_string(breadcrumb, "sid").ok(),
                },
                value: FloatValueK::Static(
                    obj.extract_number(breadcrumb, "k")?,
                ),
            }
        };
        breadcrumb.exit();
        Ok(number)
    }
}
