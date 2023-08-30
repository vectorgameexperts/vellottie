use super::position_keyframe::PositionKeyframe;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::int_boolean::BoolInt,
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

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
    #[serde(rename = "k")]
    pub value: PositionValueK,
}

/// The possible values of "k" in a [`Position`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PositionValueK {
    Animated(Vec<PositionKeyframe>),
    Static([Number; 2]),
}

impl Position {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::Position);
        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        let vector = if animated == BoolInt::True {
            todo!();
        } else {
            Position {
                property_index: obj.extract_number(breadcrumb, "ix").ok(),
                animated: obj.extract_bool_int(breadcrumb, "a")?,
                expression: obj.extract_string(breadcrumb, "x").ok(),
                length: obj.extract_number(breadcrumb, "l").ok(),
                value: PositionValueK::Static(obj.extract_type(
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
