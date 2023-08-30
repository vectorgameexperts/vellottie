use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::schema::helpers::int_boolean::BoolInt;

/// An animatable property that holds an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AnimatedProperty {
    /// Property Index
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<Number>,
    /// Whether the property is animated.
    #[serde(rename = "a")]
    pub animated: BoolInt,
    /// Expression for the property.
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// Slot ID
    #[serde(rename = "sid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
}
