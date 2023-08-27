pub(crate) mod common;
pub mod ellipse;
pub mod group;
pub mod polystar;
pub mod rectangle;

use self::common::ShapeProperties;
use self::rectangle::RectangleShape;
use crate::breadcrumb::Breadcrumb;
use crate::models::layer::animated_properties::AnimatedVector;
use crate::util::MapExt;
use crate::{error::ValueType, Error};
use ellipse::EllipseShape;
use group::GroupShape;
use serde::{Deserialize, Serialize};

use super::layer::animated_properties::AnimatedNumber;

/// Lottie considers everything related to vector data as a "shape". All shapes share the properties in `shapes::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Shape {
    /// A group is a shape that can contain other shapes (including other groups)
    Group(GroupShape),
    /// A rectangle, defined by its center point and size.
    Rectangle(RectangleShape),
    /// An ellipse, defined by its center point and width and height.
    Ellipse(EllipseShape),
    // TODO: model other shapes
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ShapeType {
    #[serde(rename = "rc")]
    Rectangle,
    #[serde(rename = "el")]
    Ellipse,
    #[serde(rename = "sr")]
    PolyStar,
    #[serde(rename = "sh")]
    Path,
    #[serde(rename = "fl")]
    Fill,
    #[serde(rename = "st")]
    Stroke,
    #[serde(rename = "gf")]
    GradientFill,
    #[serde(rename = "gs")]
    GradientStroke,
    #[serde(rename = "no")]
    NoStyle,
    #[serde(rename = "gr")]
    Group,
    #[serde(rename = "tr")]
    Transform,
    #[serde(rename = "rp")]
    Repeater,
    #[serde(rename = "tm")]
    Trim,
    #[serde(rename = "rd")]
    RoundedCorners,
    #[serde(rename = "pb")]
    PuckerBloat,
    #[serde(rename = "mm")]
    Merge,
    #[serde(rename = "tw")]
    Twist,
    #[serde(rename = "op")]
    OffsetPath,
    #[serde(rename = "zz")]
    ZigZag,
}

impl Shape {
    pub fn from_json(breadcrumb: &mut Breadcrumb, v: &serde_json::Value) -> Result<Shape, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Shape,
        })?;
        let name = root.extract_string(breadcrumb, "nm").ok();
        breadcrumb.enter(name.clone().unwrap_or("(unnamed shape)".to_string()));

        let properties = ShapeProperties::from_obj(breadcrumb, root)?;

        let shape = match &properties.shape_type {
            ShapeType::Ellipse => Shape::Ellipse(EllipseShape {
                properties,
                position: AnimatedVector::from_object(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "p")?,
                )?,
                size: AnimatedVector::from_object(breadcrumb, &root.extract_obj(breadcrumb, "s")?)?,
            }),
            ShapeType::Rectangle => Shape::Rectangle(RectangleShape {
                properties,
                position: AnimatedVector::from_object(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "p")?,
                )?,
                size: AnimatedVector::from_object(breadcrumb, &root.extract_obj(breadcrumb, "s")?)?,
                rounded_corner_radius: AnimatedNumber::from_object(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "r")?,
                )?,
            }),
            _ => todo!(),
        };

        breadcrumb.exit();
        Ok(shape)
    }
}