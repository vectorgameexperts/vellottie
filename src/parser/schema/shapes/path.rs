use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::{
    breadcrumb::Breadcrumb,
    schema::animated_properties::shape_property::ShapeProperty, util::MapExt,
    Error,
};

use super::shape::Shape;

/// Animatable Bezier curve
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PathShape {
    #[serde(flatten)]
    pub shape: Shape,

    /// Bezier path
    #[serde(rename = "ks")]
    pub shape_property: ShapeProperty,
}

impl PathShape {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let shape = Shape::from_obj(breadcrumb, obj)?;
        let shape_property = obj
            .extract_obj(breadcrumb, "ks")
            .and_then(|obj| ShapeProperty::from_obj(breadcrumb, &obj))?;
        Ok(Self {
            shape,
            shape_property,
        })
    }
}
