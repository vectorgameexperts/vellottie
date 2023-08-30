use serde::{Deserialize, Serialize};

use crate::parser::schema::{
    animated_properties::value::FloatValue, constants::composite::Composite,
};

/// Duplicates previous shapes in a group

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RepeaterShape {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,

    /// Number of copies
    #[serde(rename = "c")]
    pub copies: FloatValue,

    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<FloatValue>,

    /// Stacking order
    #[serde(rename = "m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite: Option<Composite>,

    /// Transform applied to each copy
    #[serde(rename = "tr")]
    pub transform: (), // todo repeater transform
}
