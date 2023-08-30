use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::int_boolean::BoolInt,
    util::MapExt,
    Error,
};

use super::position_keyframe::PositionKeyframe;

/// An animatable property to represent a position in space
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Position {
    /// The index of the property.
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<Number>,
    /// Whether the property is animated
    #[serde(rename = "a")]
    pub animated: BoolInt,
    /// The expression for the property.
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// Number of components in the value arrays.
    /// If present, values will be truncated or expanded to match this length when accessed from expressions.
    #[serde(rename = "l")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Number>,
    /// The value variant (Animated or Static).
    #[serde(flatten)]
    pub value: PositionValue,
}

/// Animated value variant containing keyframes.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PositionAnimatedValue {
    /// Array of keyframes.
    #[serde(rename = "k")]
    animated_value: Vec<PositionKeyframe>,
}

/// Static value variant containing an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PositionStaticValue {
    /// Array of static values.
    #[serde(rename = "k")]
    pub static_value: [Number; 2],
}

/// Represents the two possible value variants for the animated position property.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PositionValue {
    Animated(PositionAnimatedValue),
    Static(PositionStaticValue),
}

impl Position {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::AnimatedVector);
        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        let vector = if animated == BoolInt::True {
            todo!();
        } else {
            Position {
                property_index: obj.extract_number(breadcrumb, "ix").ok(),
                animated: obj.extract_bool_int(breadcrumb, "a")?,
                expression: obj.extract_string(breadcrumb, "x").ok(),
                length: obj.extract_number(breadcrumb, "l").ok(),
                value: PositionValue::Static(PositionStaticValue {
                    static_value: obj.extract_type(
                        breadcrumb,
                        "k",
                        ValueType::Scalar2d,
                    )?,
                }),
            }
        };
        breadcrumb.exit();
        Ok(vector)
    }
}
