use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::{bezier::Bezier, int_boolean::BoolInt},
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use super::shape_keyframe::ShapeKeyframe;

/// An animatable property that holds a Bezier
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeProperty {
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
    ///
    #[serde(rename = "k")]
    pub value: ShapePropertyK,
}

/// The possible values of "k" in a [`ShapeProperty`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapePropertyK {
    Animated(Vec<ShapeKeyframe>),
    Static(Bezier),
}

impl ShapeProperty {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::ShapeProperty);
        let property_index = obj.extract_number(breadcrumb, "ix").ok();
        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        let expression = obj.extract_string(breadcrumb, "x").ok();
        let value = if animated == BoolInt::True {
            let arr = obj.extract_arr(breadcrumb, "k")?;
            breadcrumb.enter(ValueType::ShapeKeyframe, Some("k"));
            let mut values = vec![];
            for v in arr {
                let keyframe_obj =
                    v.as_object().ok_or(Error::UnexpectedChild {
                        breadcrumb: breadcrumb.clone(),
                        expected: ValueType::Object,
                    })?;
                values.push(ShapeKeyframe::from_obj(breadcrumb, keyframe_obj)?);
            }
            breadcrumb.exit();
            ShapePropertyK::Animated(values)
        } else {
            ShapePropertyK::Static(Bezier::from_obj(
                breadcrumb,
                &obj.extract_obj(breadcrumb, "k")?,
            )?)
        };
        breadcrumb.exit();
        Ok(Self {
            property_index,
            animated,
            expression,
            value,
        })
    }
}
