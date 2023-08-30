use serde::{Deserialize, Serialize};

use crate::parser::schema::{
    animated_properties::value::Scalar, constants::composite::Composite,
};

/// Duplicates previous shapes in a group

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RepeaterShape {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,

    /// Number of copies
    #[serde(rename = "c")]
    pub copies: Scalar,

    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Scalar>,

    /// Stacking order
    #[serde(rename = "m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite: Option<Composite>,

    /// Transform applied to each copy
    #[serde(rename = "tr")]
    pub transform: (), // todo repeater transform
}
