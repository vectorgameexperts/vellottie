use crate::import::properties::{conv_color, conv_multi_point, conv_size};
use crate::parser::schema::animated_properties::gradient_colors::GradientColors;
use crate::parser::schema::constants::gradient_type::GradientType;
use crate::parser::{self, Lottie};
use crate::runtime::model::animated::Position;
use crate::runtime::model::{
    animated, Draw, GroupTransform, SplineToPath, Time, Value,
};
use crate::runtime::{self, Composition};
use parser::schema;
use std::collections::HashMap;
use vello::kurbo::Point;
use vello::peniko::{self, BlendMode, Mix};

use self::defaults::{
    FLOAT_VALUE_ONE_HUNDRED, FLOAT_VALUE_ZERO, MULTIDIM_ONE, POSITION_ZERO,
};
use self::layers::conv_layer;
use self::properties::{conv_pos_point, conv_scalar, conv_vec2};

mod defaults;
mod layers;
mod properties;

pub trait NumberExt {
    fn unwrap_f32(&self) -> f32;
    fn unwrap_f64(&self) -> f64;
    fn unwrap_u32(&self) -> u32;
}

impl NumberExt for serde_json::Number {
    fn unwrap_f32(&self) -> f32 {
        self.as_f64().expect("Could not get float from JSON Number") as f32
    }

    fn unwrap_f64(&self) -> f64 {
        self.as_f64().expect("Could not get float from JSON Number")
    }

    fn unwrap_u32(&self) -> u32 {
        self.as_u64()
            .expect("Could not get unsigned integer from JSON Number")
            as u32
    }
}

pub fn import_composition(
    source: impl AsRef<[u8]>,
) -> Result<Composition, Box<dyn std::error::Error>> {
    let source = Lottie::from_slice(source.as_ref())?;
    let mut target = Composition {
        frames: source.in_point.unwrap_f32()..source.out_point.unwrap_f32(),
        frame_rate: source.frame_rate.unwrap_f32(),
        width: source.width.unwrap_u32(),
        height: source.height.unwrap_u32(),
        assets: Default::default(),
        layers: Default::default(),
    };

    // Collect assets and layers
    let mut idmap: HashMap<usize, usize> = HashMap::default();
    if let Some(assets) = source.assets {
        for asset in assets {
            match asset {
                parser::schema::assets::AnyAsset::Precomposition(precomp) => {
                    idmap.clear();
                    let mut layers = vec![];
                    let mut mask_layer = None;
                    for layer in precomp.composition.layers.iter() {
                        let index = layers.len();
                        if let Some((mut layer, id, mask_blend)) =
                            conv_layer(layer)
                        {
                            if let (Some(mask_blend), Some(mask_layer)) =
                                (mask_blend, mask_layer.take())
                            {
                                layer.mask_layer =
                                    Some((mask_blend, mask_layer));
                            }
                            if layer.is_mask {
                                mask_layer = Some(index);
                            }
                            idmap.insert(id, index);
                            layers.push(layer);
                        }
                    }
                    for layer in &mut layers {
                        if let Some(parent) = layer.parent {
                            layer.parent = idmap.get(&parent).copied();
                        }
                    }
                    target.assets.insert(precomp.asset.id.clone(), layers);
                }
                asset => {
                    unimplemented!("asset {:?} is not yet implemented", asset)
                }
            }
        }
    }

    idmap.clear();
    let mut layers = vec![];
    let mut mask_layer = None;
    for layer in &source.layers {
        let index = layers.len();
        if let Some((mut layer, id, mask_blend)) = conv_layer(layer) {
            if let (Some(mask_blend), Some(mask_layer)) =
                (mask_blend, mask_layer.take())
            {
                layer.mask_layer = Some((mask_blend, mask_layer));
            }
            if layer.is_mask {
                mask_layer = Some(index);
            }
            idmap.insert(id, index);
            layers.push(layer);
        }
    }
    for layer in &mut layers {
        if let Some(parent) = layer.parent {
            layer.parent = idmap.get(&parent).copied();
        }
    }
    target.layers = layers;

    Ok(target)
}

fn conv_shape_transform(
    value: &parser::schema::shapes::transform::TransformShape,
) -> GroupTransform {
    let rotation_in = match &value.transform.rotation {
        Some(any_trans) => match any_trans {
            parser::schema::helpers::transform::AnyTransformR::Rotation(float_value) => float_value,
            // todo: need to actually handle split rotations
            parser::schema::helpers::transform::AnyTransformR::SplitRotation { .. } => todo!("split rotation"),
        },
        None => &FLOAT_VALUE_ZERO,
    };
    let position_in = match &value.transform.position {
        schema::helpers::transform::AnyTransformP::Position(position) => {
            position
        }
        schema::helpers::transform::AnyTransformP::SplitPosition(_) => {
            // todo: split vectors
            todo!("split position");
        }
    };

    let transform = animated::Transform {
        anchor: conv_pos_point(
            value
                .transform
                .anchor_point
                .as_ref()
                .unwrap_or(&POSITION_ZERO),
        ),
        position: Position::Value(conv_pos_point(position_in)),
        scale: conv_vec2(
            value.transform.scale.as_ref().unwrap_or(&MULTIDIM_ONE),
        ),
        rotation: conv_scalar(rotation_in),
        skew: conv_scalar(
            value.transform.skew.as_ref().unwrap_or(&FLOAT_VALUE_ZERO),
        ),
        skew_angle: conv_scalar(
            value
                .transform
                .skew_axis
                .as_ref()
                .unwrap_or(&FLOAT_VALUE_ZERO),
        ),
    };

    let opacity = conv_scalar(
        value
            .transform
            .opacity
            .as_ref()
            .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
    );

    GroupTransform {
        transform: transform.to_model(),
        opacity,
    }
}

fn conv_gradient_colors(value: &GradientColors) -> runtime::model::ColorStops {
    use schema::animated_properties::animated_property::AnimatedPropertyK::*;

    let count = value.count.unwrap_u32() as usize;
    match &value.colors.animated_property.value {
        Static(value) => {
            let mut stops = runtime::model::fixed::ColorStops::new();
            for chunk in value.chunks_exact(4) {
                stops.push(
                    (
                        chunk[0].unwrap_f32(),
                        runtime::model::fixed::Color::rgba(
                            chunk[1].unwrap_f64(),
                            chunk[2].unwrap_f64(),
                            chunk[3].unwrap_f64(),
                            1.0,
                        ),
                    )
                        .into(),
                )
            }
            runtime::model::ColorStops::Fixed(stops)
        }
        AnimatedValue(animated) => {
            let mut frames = vec![];
            let mut values = vec![];
            for value in animated {
                frames.push(Time {
                    frame: value.base.time.unwrap_f32(),
                });
                values.push(
                    value
                        .value
                        .iter()
                        .map(|x| x.unwrap_f32())
                        .collect::<Vec<_>>(),
                );
            }
            runtime::model::ColorStops::Animated(animated::ColorStops {
                frames,
                values,
                count,
            })
        }
    }
}

fn conv_draw(value: &schema::shapes::AnyShape) -> Option<runtime::model::Draw> {
    use peniko::{Cap, Join};
    use schema::constants::line_cap::LineCap;
    use schema::constants::line_join::LineJoin;
    use schema::shapes::AnyShape;

    match value {
        AnyShape::Fill(value) => {
            let color = conv_color(&value.color);
            let brush = animated::Brush::Solid(color).to_model();
            let opacity = conv_scalar(
                value.opacity.as_ref().unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
            );
            Some(runtime::model::Draw {
                stroke: None,
                brush,
                opacity,
            })
        }
        AnyShape::Stroke(value) => {
            let stroke = animated::Stroke {
                width: conv_scalar(&value.stroke_width),
                join: match value.line_join.as_ref().unwrap_or(&LineJoin::Bevel)
                {
                    LineJoin::Bevel => Join::Bevel,
                    LineJoin::Round => Join::Round,
                    LineJoin::Miter => Join::Miter,
                },
                miter_limit: value
                    .miter_limit
                    .as_ref()
                    .map(|number| number.unwrap_f32()),
                cap: match value.line_cap.as_ref().unwrap_or(&LineCap::Butt) {
                    LineCap::Butt => Cap::Butt,
                    LineCap::Round => Cap::Round,
                    LineCap::Square => Cap::Square,
                },
            };
            let color = conv_color(&value.stroke_color);
            let brush = animated::Brush::Solid(color).to_model();
            let opacity = conv_scalar(&value.opacity);
            Some(runtime::model::Draw {
                stroke: Some(stroke.to_model()),
                brush,
                opacity,
            })
        }
        AnyShape::GradientFill(value) => {
            let is_radial = matches!(
                value.gradient.gradient_type,
                Some(GradientType::Radial)
            );
            let start_point = conv_multi_point(&value.gradient.start_point);
            let end_point = conv_multi_point(&value.gradient.end_point);
            let gradient = animated::Gradient {
                is_radial,
                start_point,
                end_point,
                stops: conv_gradient_colors(&value.gradient.colors),
            };
            let brush = animated::Brush::Gradient(gradient).to_model();
            Some(Draw {
                stroke: None,
                brush,
                opacity: Value::Fixed(100.0),
            })
        }
        // todo:
        // Shape::GradientStroke(value) => {
        //     let stroke = animated::Stroke {
        //         width: conv_scalar(&value.stroke_width),
        //         join: match value.line_join {
        //             LineJoin::Bevel => Join::Bevel,
        //             LineJoin::Round => Join::Round,
        //             LineJoin::Miter => Join::Miter,
        //         },
        //         miter_limit: value.miter_limit.map(|x| x as f32),
        //         cap: match value.line_cap {
        //             LineCap::Butt => Cap::Butt,
        //             LineCap::Round => Cap::Round,
        //             LineCap::Square => Cap::Square,
        //         },
        //     };
        //     let is_radial = matches!(value.ty, GradientType::Radial);
        //     let start_point = conv_point(&value.start_point);
        //     let end_point = conv_point(&value.end_point);
        //     let gradient = animated::Gradient {
        //         is_radial,
        //         start_point,
        //         end_point,
        //         stops: conv_gradient_colors(&value.gradient_colors),
        //     };
        //     let brush = animated::Brush::Gradient(gradient).to_model();
        //     Some(Draw {
        //         stroke: Some(stroke.to_model()),
        //         brush,
        //         opacity: Value::Fixed(100.0),
        //     })
        // }
        _ => None,
    }
}

fn conv_shape(
    value: &parser::schema::shapes::AnyShape,
) -> Option<crate::runtime::model::Shape> {
    if let Some(draw) = conv_draw(value) {
        return Some(crate::runtime::model::Shape::Draw(draw));
    } else if let Some(geometry) = conv_geometry(value) {
        return Some(crate::runtime::model::Shape::Geometry(geometry));
    }

    match value {
        schema::shapes::AnyShape::Group(value) => {
            let mut shapes = vec![];
            let mut group_transform = None;
            for item in &value.shapes {
                match item {
                    schema::shapes::AnyShape::Transform(transform) => {
                        group_transform = Some(conv_shape_transform(transform));
                    }
                    _ => {
                        if let Some(shape) = conv_shape(item) {
                            shapes.push(shape);
                        }
                    }
                }
            }
            if !shapes.is_empty() {
                Some(crate::runtime::model::Shape::Group(
                    shapes,
                    group_transform,
                ))
            } else {
                None
            }
        }
        // todo: implement repeater shape
        // shapes::Shape::Repeater(value) => {
        //     let repeater = animated::Repeater {
        //         copies: conv_scalar(&value.copies),
        //         offset: conv_scalar(&value.offset),
        //         anchor_point: conv_point(&value.transform.anchor_point),
        //         position: conv_point(&value.transform.position),
        //         rotation: conv_scalar(&value.transform.rotation),
        //         scale: conv_vec2(&value.transform.scale),
        //         start_opacity: conv_scalar(&value.transform.start_opacity),
        //         end_opacity: conv_scalar(&value.transform.end_opacity),
        //     };
        //     Some(Shape::Repeater(repeater.to_model()))
        // }
        _ => None,
    }
}

fn conv_geometry(
    value: &schema::shapes::AnyShape,
) -> Option<crate::runtime::model::Geometry> {
    use schema::shapes::AnyShape;
    match value {
        AnyShape::Ellipse(value) => {
            let ellipse = animated::Ellipse {
                is_ccw: false, // todo: lottie schema does not have a field for this (anymore?)
                position: conv_pos_point(&value.position),
                size: conv_size(&value.size),
            };
            Some(crate::runtime::model::Geometry::Ellipse(ellipse))
        }
        AnyShape::Rectangle(value) => {
            let rect = animated::Rect {
                is_ccw: false, // todo: lottie schema does not have a field for this (anymore?)
                position: conv_pos_point(&value.position),
                size: conv_size(&value.size),
                corner_radius: conv_scalar(&value.rounded_corner_radius),
            };
            Some(crate::runtime::model::Geometry::Rect(rect))
        }
        AnyShape::Path(value) => conv_shape_geometry(&value.shape_property),
        // todo: generic shape
        _ => None,
    }
}

fn conv_shape_geometry(
    value: &schema::animated_properties::shape_property::ShapeProperty,
) -> Option<runtime::model::Geometry> {
    use schema::animated_properties::shape_property::ShapePropertyK::*;
    let mut is_closed = false;
    match &value.value {
        Static(value) => {
            let (points, is_closed) = conv_spline(value);
            let mut path = vec![];
            points.as_slice().to_path(is_closed, &mut path);
            Some(runtime::model::Geometry::Fixed(path))
        }
        Animated(animated) => {
            let mut frames = vec![];
            let mut values = vec![];
            for value in animated {
                frames.push(Time {
                    frame: value.base.time.unwrap_f32(),
                });
                let (points, is_frame_closed) =
                    conv_spline(value.start.get(0)?);
                values.push(points);
                is_closed |= is_frame_closed;
            }
            Some(runtime::model::Geometry::Spline(animated::Spline {
                is_closed,
                times: frames,
                values,
            }))
        }
    }
}

fn conv_spline(value: &schema::helpers::bezier::Bezier) -> (Vec<Point>, bool) {
    use core::iter::repeat;
    let mut points = Vec::with_capacity(value.vertices.len() * 3);
    let is_closed = value.closed.unwrap_or(false);

    for ((v, i), o) in value
        .vertices
        .iter()
        .zip(value.in_tangents.iter().chain(repeat(&[
            serde_json::Number::from(0),
            serde_json::Number::from(0),
        ])))
        .zip(value.out_tangents.iter().chain(repeat(&[
            serde_json::Number::from(0),
            serde_json::Number::from(0),
        ])))
    {
        points.push((v[0].unwrap_f64(), v[1].unwrap_f64()).into());
        points.push((i[0].unwrap_f64(), i[1].unwrap_f64()).into());
        points.push((o[0].unwrap_f64(), o[1].unwrap_f64()).into());
    }
    (points, is_closed)
}

fn conv_blend_mode(
    value: &crate::parser::schema::constants::blend_mode::BlendMode,
) -> Option<BlendMode> {
    use crate::parser::schema::constants::blend_mode::BlendMode::*;

    Some(match value {
        Normal => return None,
        Multiply => BlendMode::from(Mix::Multiply),
        Screen => BlendMode::from(Mix::Screen),
        Overlay => BlendMode::from(Mix::Overlay),
        Darken => BlendMode::from(Mix::Darken),
        Lighten => BlendMode::from(Mix::Lighten),
        ColorDodge => BlendMode::from(Mix::ColorDodge),
        ColorBurn => BlendMode::from(Mix::ColorBurn),
        HardLight => BlendMode::from(Mix::HardLight),
        SoftLight => BlendMode::from(Mix::SoftLight),
        Difference => BlendMode::from(Mix::Difference),
        Exclusion => BlendMode::from(Mix::Exclusion),
        Hue => BlendMode::from(Mix::Hue),
        Saturation => BlendMode::from(Mix::Saturation),
        Color => BlendMode::from(Mix::Color),
        Luminosity => BlendMode::from(Mix::Luminosity),
        Add => unimplemented!(),
        HardMix => unimplemented!(),
    })
}
