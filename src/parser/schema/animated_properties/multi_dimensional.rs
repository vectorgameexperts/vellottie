use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::int_boolean::BoolInt,
    util::MapExt,
    Error,
};

use super::{animated_property::AnimatedProperty, keyframe::Keyframe};

/// Animated Vector
///
/// An animatable property that holds an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MultiDimensionalValue {
    Animated(Vec<Keyframe>),
    Static([Number; 2]),
}

/// Static value variant of a float component array.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MultiDimensional {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty,
    /// A single component array.
    #[serde(rename = "k")]
    pub value: MultiDimensionalValue,
}

impl MultiDimensional {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::AnimatedVector);
        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        let vector = if animated == BoolInt::True {
            todo!();
        } else {
            MultiDimensional {
                animated_property: AnimatedProperty {
                    property_index: obj.extract_number(breadcrumb, "ix").ok(),
                    animated: obj.extract_bool_int(breadcrumb, "a")?,
                    expression: obj.extract_string(breadcrumb, "x").ok(),
                    slot_id: obj.extract_string(breadcrumb, "sid").ok(),
                },
                value: MultiDimensionalValue::Static(obj.extract_type(
                    breadcrumb,
                    "k",
                    ValueType::Scalar2d,
                )?),
            }
        };
        breadcrumb.exit();
        Ok(vector)
    }
}
