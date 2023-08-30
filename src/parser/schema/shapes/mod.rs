pub(crate) mod common;
pub mod ellipse;
pub mod enumerations;
pub mod fill;
pub mod group;
pub mod merge;
pub mod offset_path;
pub mod polystar;
pub mod pucker_bloat;
pub mod rectangle;
pub mod repeater;
pub mod repeater_transform;
pub mod shape;
pub mod shape_element;
pub mod stroke;
pub mod transform;
// todo pub mod trim;
// todo pub mod path;
// todo pub mod gradient_stroke;
// todo pub mod stroke_dash;
// todo pub mod shape_list;
// todo pub mod zig_zag;
// todo pub mod no_style;
// todo pub mod base_stroke;
// todo pub mod twist;
// todo pub mod rounded_corners;
// todo pub mod gradient_fill;
// todo pub mod gradient;
// todo pub mod modifier;

use self::{
    fill::FillShape, merge::MergeShape, offset_path::OffsetPathShape,
    pucker_bloat::PuckerBloatShape, rectangle::RectangleShape,
    repeater::RepeaterShape, repeater_transform::RepeaterTransformShape,
    shape::GenericShape, shape_element::ShapeElementShape, stroke::StrokeShape,
    transform::TransformShape,
};
use crate::parser::breadcrumb::Breadcrumb;
use crate::parser::{breadcrumb::ValueType, util::MapExt, Error};
use ellipse::EllipseShape;
use group::GroupShape;
use serde::{Deserialize, Serialize};

pub use self::common::ShapeProperties;

use super::animated_properties::multi_dimensional::MultiDimensional;
use super::animated_properties::position::Position;
use super::animated_properties::value::FloatValue;
use super::helpers::transform::Transform;

/// Lottie considers everything related to vector data as a "shape". All shapes
/// share the properties in `shapes::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Shape {
    /// A group is a shape that can contain other shapes (including other
    /// groups)
    Group(GroupShape),
    /// A rectangle, defined by its center point and size.
    Rectangle(RectangleShape),
    /// An ellipse, defined by its center point and width and height.
    Ellipse(EllipseShape),
    Transform(TransformShape),
    Stroke(StrokeShape),
    // TODO: model other shapes
    PuckerBloat(PuckerBloatShape),
    Merge(MergeShape),
    Repeater(RepeaterShape),
    OffsetPath(OffsetPathShape),
    Fill(FillShape),
    RepeaterTransform(RepeaterTransformShape),
    ShapeElement(ShapeElementShape),
    Shape(GenericShape),
    // todo Trim(trim),
    // todo Path(path),
    // todo GradientStroke(gradient_stroke),
    // todo StrokeDash(stroke_dash),
    // todo ShapeList(shape_list),
    // todo ZigZag(zig_zag),
    // todo no_style(no_style),
    // todo BaseStroke(base_stroke),
    // todo Twist(twist),
    // todo RoundedCorners(rounded_corners),
    // todo GradientFill(gradient_fill),
    // todo gradient(gradient),
    // todo modifier(modifier),
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
    pub fn from_json(
        breadcrumb: &mut Breadcrumb,
        v: &serde_json::Value,
    ) -> Result<Shape, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Shape,
        })?;
        let name = root.extract_string(breadcrumb, "nm").ok();
        breadcrumb.enter(ValueType::Shape, name);

        // Extract
        let properties = ShapeProperties::from_obj(breadcrumb, root)?;
        let shape = match &properties.shape_type {
            ShapeType::Ellipse => Shape::Ellipse(EllipseShape {
                properties,
                position: Position::from_obj(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "p")?,
                )?,
                size: MultiDimensional::from_obj(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "s")?,
                )?,
            }),
            ShapeType::Rectangle => Shape::Rectangle(RectangleShape {
                properties,
                position: Position::from_obj(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "p")?,
                )?,
                size: MultiDimensional::from_obj(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "s")?,
                )?,
                rounded_corner_radius: FloatValue::from_obj(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "r")?,
                )?,
            }),
            ShapeType::Group => Shape::Group(GroupShape {
                properties,
                num_properties: root.extract_number(breadcrumb, "np").ok(),
                shapes: {
                    let mut shapes = vec![];
                    let json_shapes = root.extract_arr(breadcrumb, "it")?;
                    breadcrumb.enter(ValueType::Array, Some("it"));
                    for v in json_shapes {
                        let shape = Shape::from_json(breadcrumb, &v)?;
                        shapes.push(shape);
                    }
                    breadcrumb.exit();
                    shapes
                },
            }),
            ShapeType::Transform => Shape::Transform(TransformShape {
                properties,
                transform: Transform::from_obj(breadcrumb, root)?,
            }),
            ShapeType::Stroke => Shape::Stroke(StrokeShape {
                properties,
                line_cap: root
                    .extract_type(breadcrumb, "key", ValueType::EnumInt)
                    .ok(),
                line_join: root
                    .extract_type(breadcrumb, "key", ValueType::EnumInt)
                    .ok(),
                miter_limit: root.extract_number(breadcrumb, "key").ok(),
                miter_limit_alt: root
                    .extract_type(breadcrumb, "key", ValueType::Scalar2d)
                    .ok(),
                opacity: todo!(),
                stroke_width: todo!(),
                dash_array: todo!(),
                stroke_color: todo!(),
            }),
            other_shape => {
                todo!("Shape {:?} not yet implemented", other_shape)
            }
        };

        breadcrumb.exit();
        Ok(shape)
    }
}
