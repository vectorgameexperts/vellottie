use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::int_boolean::BoolInt,
    util::MapExt,
    Error,
};

use super::animated_property::{AnimatedProperty, AnimatedPropertyBase};

/// Animated Number
///
/// An animatable property that holds a float.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Scalar {
    Animated(AnimatedProperty),
    Static(ScalarValue),
}

/// Static value variant of a single float number.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScalarValue {
    #[serde(flatten)]
    pub animated_property_base: AnimatedPropertyBase,
    /// A single value.
    #[serde(rename = "k")]
    pub value: Number,
}

impl Scalar {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::AnimatedNumber);

        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        let number = if animated == BoolInt::True {
            todo!();
        } else {
            Scalar::Static(ScalarValue {
                animated_property_base: AnimatedPropertyBase {
                    property_index: obj.extract_number(breadcrumb, "ix").ok(),
                    animated: obj.extract_bool_int(breadcrumb, "a")?,
                    expression: obj.extract_string(breadcrumb, "x").ok(),
                    slot_id: obj.extract_string(breadcrumb, "sid").ok(),
                },
                value: obj.extract_number(breadcrumb, "k")?,
            })
        };
        breadcrumb.exit();
        Ok(number)
    }
}
