//! Shapes - https://lottiefiles.github.io/lottie-docs/shapes/

use super::common;
use crate::breadcrumb::Breadcrumb;
use crate::models::shapes::Shape;
use crate::{
    util::{self},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use util::MapExt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeLayer {
    #[serde(flatten)]
    pub properties: common::LayerProperties,
    /// Has an array of shapes
    pub shapes: Vec<Shape>,
}

impl ShapeLayer {
    pub fn from_properties_and_object(
        breadcrumb: &mut Breadcrumb,
        properties: common::LayerProperties,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let mut shapes = vec![];
        for s in obj.extract_arr(breadcrumb, "shapes")? {
            breadcrumb.enter("shapes");
            let shape = Shape::from_json(breadcrumb, &s)?;
            shapes.push(shape);
            breadcrumb.exit();
        }

        Ok(ShapeLayer { properties, shapes })
    }
}
