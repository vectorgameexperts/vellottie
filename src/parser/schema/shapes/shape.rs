use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::constants::shape_direction::ShapeDirection,
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::shape_element::ShapeElement;

/// Drawable shape
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Shape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    /// Direction the shape is drawn as, mostly relevant when using trim path
    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ShapeDirection>,
}

impl Shape {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let shape_element = ShapeElement::from_obj(breadcrumb, obj)?;
        let direction =
            obj.extract_type(breadcrumb, "d", ValueType::EnumInt).ok();
        Ok(Self {
            shape_element,
            direction,
        })
    }
}
