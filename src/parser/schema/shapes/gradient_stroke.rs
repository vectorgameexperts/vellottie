use crate::parser::{breadcrumb::Breadcrumb, Error};

use super::{
    base_stroke::BaseStroke, gradient::Gradient, shape_element::ShapeElement,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientStrokeShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    #[serde(flatten)]
    pub base_stroke: BaseStroke,

    #[serde(flatten)]
    pub gradient: Gradient,
}

impl GradientStrokeShape {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let shape_element = ShapeElement::from_obj(breadcrumb, obj)?;
        let base_stroke = BaseStroke::from_obj(breadcrumb, obj)?;
        let gradient = Gradient::from_obj(breadcrumb, obj)?;

        Ok(Self {
            shape_element,
            base_stroke,
            gradient,
        })
    }
}
