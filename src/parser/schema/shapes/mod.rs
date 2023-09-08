pub mod ellipse;
pub mod fill;
pub mod group;
pub mod merge;
pub mod offset_path;
pub mod path;
pub mod polystar;
pub mod pucker_bloat;
pub mod rectangle;
pub mod repeater;
pub mod repeater_transform;
pub mod shape;
pub mod shape_element;
pub mod stroke;
pub mod stroke_dash;
pub mod transform;
pub mod trim;
// todo pub mod gradient_stroke;
// todo pub mod stroke_dash;
// todo pub mod shape_list;
// todo pub mod zig_zag;
// todo pub mod no_style;
// todo pub mod base_stroke;
// todo pub mod twist;
// todo pub mod rounded_corners;
pub mod gradient;
pub mod gradient_fill;
// todo pub mod modifier;

use self::gradient::Gradient;
use self::gradient_fill::GradientFillShape;
use self::path::PathShape;
use self::shape_element::ShapeElement;
use self::{
    fill::FillShape, merge::MergeShape, offset_path::OffsetPathShape,
    pucker_bloat::PuckerBloatShape, rectangle::RectangleShape,
    repeater::RepeaterShape, stroke::StrokeShape, transform::TransformShape,
    trim::TrimShape,
};
use crate::parser::breadcrumb::Breadcrumb;
use crate::parser::{breadcrumb::ValueType, util::MapExt, Error};
use ellipse::EllipseShape;
use group::GroupShape;
use serde::{Deserialize, Serialize};

use super::animated_properties::color_value::ColorValue;
use super::animated_properties::gradient_colors::GradientColors;
use super::animated_properties::multi_dimensional::MultiDimensional;
use super::animated_properties::position::Position;
use super::animated_properties::shape_property::ShapeProperty;
use super::animated_properties::value::FloatValue;
use super::helpers::transform::Transform;

/// Lottie considers everything related to vector data as a "shape". All shapes
/// share the properties in `shapes::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "ty")]
pub enum AnyShape {
    /// A group is a shape that can contain other shapes (including other
    /// groups)
    #[serde(rename = "gr")]
    Group(GroupShape),
    /// A rectangle, defined by its center point and size.
    #[serde(rename = "rc")]
    Rectangle(RectangleShape),
    /// An ellipse, defined by its center point and width and height.
    #[serde(rename = "el")]
    Ellipse(EllipseShape),
    #[serde(rename = "tr")]
    Transform(TransformShape),
    #[serde(rename = "st")]
    Stroke(StrokeShape),
    #[serde(rename = "pb")]
    PuckerBloat(PuckerBloatShape),
    #[serde(rename = "mm")]
    Merge(MergeShape),
    #[serde(rename = "rp")]
    Repeater(RepeaterShape),
    #[serde(rename = "op")]
    OffsetPath(OffsetPathShape),
    #[serde(rename = "fl")]
    Fill(FillShape),
    #[serde(rename = "tm")]
    Trim(TrimShape),
    #[serde(rename = "sh")]
    Path(PathShape),
    #[serde(rename = "gf")]
    GradientFill(GradientFillShape),
    // TODO: model other shapes
    // todo GradientStroke(gradient_stroke),
    // todo ZigZag(zig_zag),
    // todo no_style(no_style),
    // todo Twist(twist),
    // todo RoundedCorners(rounded_corners),
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

impl AnyShape {
    pub fn from_json(
        breadcrumb: &mut Breadcrumb,
        v: &serde_json::Value,
    ) -> Result<AnyShape, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Shape,
        })?;
        let name = root.extract_string(breadcrumb, "nm").ok();
        breadcrumb.enter(ValueType::Shape, name);

        // Extract
        let shape_element = ShapeElement::from_obj(breadcrumb, root)?;
        let shape_type =
            root.extract_type(breadcrumb, "ty", ValueType::EnumInt)?;
        let shape = match shape_type {
            ShapeType::Ellipse => AnyShape::Ellipse(EllipseShape {
                shape_element,
                position: Position::from_obj(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "p")?,
                )?,
                size: MultiDimensional::from_obj(
                    breadcrumb,
                    &root.extract_obj(breadcrumb, "s")?,
                )?,
            }),
            ShapeType::Rectangle => AnyShape::Rectangle(RectangleShape {
                shape_element,
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
            ShapeType::Group => AnyShape::Group(GroupShape {
                shape_element,
                num_properties: root.extract_number(breadcrumb, "np").ok(),
                shapes: {
                    let mut shapes = vec![];
                    let json_shapes = root.extract_arr(breadcrumb, "it")?;
                    breadcrumb.enter(ValueType::Array, Some("it"));
                    for v in json_shapes {
                        let shape = AnyShape::from_json(breadcrumb, &v)?;
                        shapes.push(shape);
                    }
                    breadcrumb.exit();
                    shapes
                },
            }),
            ShapeType::Transform => AnyShape::Transform(TransformShape {
                shape_element,
                transform: Transform::from_obj(breadcrumb, root)?,
            }),
            ShapeType::Stroke => AnyShape::Stroke(StrokeShape {
                shape_element,
                line_cap: root
                    .extract_type(breadcrumb, "lc", ValueType::EnumInt)
                    .ok(),
                line_join: root
                    .extract_type(breadcrumb, "lj", ValueType::EnumInt)
                    .ok(),
                miter_limit: root.extract_number(breadcrumb, "ml").ok(),
                miter_limit_alt: root
                    .extract_type(breadcrumb, "ml2", ValueType::Scalar2d)
                    .ok(),
                opacity: root
                    .extract_obj(breadcrumb, "o")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?,
                stroke_width: root
                    .extract_obj(breadcrumb, "w")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?,
                stroke_color: root
                    .extract_obj(breadcrumb, "c")
                    .and_then(|obj| ColorValue::from_obj(breadcrumb, &obj))?,
                dash_array: root
                    .extract_type(breadcrumb, "d", ValueType::EnumStr)
                    .ok(),
            }),
            ShapeType::Fill => AnyShape::Fill(FillShape {
                shape_element,
                opacity: root
                    .extract_obj(breadcrumb, "o")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
                    .ok(),
                color: root
                    .extract_obj(breadcrumb, "c")
                    .and_then(|obj| ColorValue::from_obj(breadcrumb, &obj))?,
                fill_rule: root
                    .extract_type(breadcrumb, "r", ValueType::EnumInt)
                    .ok(),
            }),
            ShapeType::Trim => AnyShape::Trim(TrimShape {
                shape_element,
                start: root
                    .extract_obj(breadcrumb, "s")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?,
                end: root
                    .extract_obj(breadcrumb, "e")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?,
                offset: root
                    .extract_obj(breadcrumb, "o")
                    .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?,
                multiple: root
                    .extract_type(breadcrumb, "m", ValueType::EnumInt)
                    .ok(),
            }),
            ShapeType::Path => AnyShape::Path(PathShape {
                shape_element,
                shape: root.extract_obj(breadcrumb, "ks").and_then(|obj| {
                    ShapeProperty::from_obj(breadcrumb, &obj)
                })?,
            }),
            ShapeType::GradientFill => {
                AnyShape::GradientFill(GradientFillShape {
                    shape_element,
                    gradient: Gradient {
                        start_point: root
                            .extract_obj(breadcrumb, "s")
                            .and_then(|obj| {
                                MultiDimensional::from_obj(breadcrumb, &obj)
                            })?,
                        end_point: root.extract_obj(breadcrumb, "e").and_then(
                            |obj| MultiDimensional::from_obj(breadcrumb, &obj),
                        )?,
                        gradient_type: root
                            .extract_type(breadcrumb, "t", ValueType::EnumInt)
                            .ok(),
                        highlight_length: root
                            .extract_obj(breadcrumb, "h")
                            .and_then(|obj| {
                                FloatValue::from_obj(breadcrumb, &obj)
                            })
                            .ok(),
                        highlight_angle: root
                            .extract_obj(breadcrumb, "a")
                            .and_then(|obj| {
                                FloatValue::from_obj(breadcrumb, &obj)
                            })
                            .ok(),
                        colors: root.extract_obj(breadcrumb, "g").and_then(
                            |obj| GradientColors::from_obj(breadcrumb, &obj),
                        )?,
                    },
                    opacity: root.extract_obj(breadcrumb, "o").and_then(
                        |obj| FloatValue::from_obj(breadcrumb, &obj),
                    )?,
                    fill_rule: root
                        .extract_type(breadcrumb, "r", ValueType::EnumInt)
                        .ok(),
                })
            }
            other_shape => {
                todo!("Shape {:?} not yet implemented", other_shape)
            }
        };

        breadcrumb.exit();
        Ok(shape)
    }
}
