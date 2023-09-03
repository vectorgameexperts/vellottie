use crate::parser::schema::animated_properties::multi_dimensional::MultiDimensional;
use crate::parser::schema::animated_properties::value::FloatValue;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use crate::parser::{self, Lottie};
use crate::runtime::model::animated::Position;
use crate::runtime::model::{
    self, animated, Content, GroupTransform, Layer, Lerp, SplineToPath, Time,
    Value,
};
use crate::runtime::{self, Composition};
use lazy_static::lazy_static;
use parser::schema;
use std::collections::HashMap;
use vello::kurbo::{Point, Size, Vec2};
use vello::peniko::{self, BlendMode, Color, Compose, Mix};

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

lazy_static! {
    static ref FLOAT_VALUE_ZERO: FloatValue = FloatValue {
        animated_property:
            schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: BoolInt::False,
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    serde_json::Number::from(0),
                ),
            },
    };
    static ref FLOAT_VALUE_ONE_HUNDRED: FloatValue = FloatValue {
        animated_property:
            schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: BoolInt::False,
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    serde_json::Number::from(100),
                ),
            },
    };

    static ref MULTIDIM_ZERO: MultiDimensional =
        MultiDimensional {
            animated_property: schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: BoolInt::False,
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    vec![serde_json::Number::from(0), serde_json::Number::from(0), serde_json::Number::from(0)],
                ),
            },
        };
    static ref MULTIDIM_ONE: MultiDimensional =
        MultiDimensional {
            animated_property: schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: BoolInt::False,
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    vec![serde_json::Number::from(1), serde_json::Number::from(1), serde_json::Number::from(1)],
                ),
            },
        };

    static ref POSITION_ZERO: schema::animated_properties::position::Position = schema::animated_properties::position::Position {
        property_index: None,
        animated: BoolInt::False,
        expression: None,
        length: None,
        value: schema::animated_properties::position::PositionValueK::Static([serde_json::Number::from(0), serde_json::Number::from(0)]),
    };

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
        } else {
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

fn conv_layer(
    source: &parser::schema::layers::AnyLayer,
) -> Option<(Layer, usize, Option<BlendMode>)> {
    let mut layer = Layer::default();
    let params;
    match source {
        // TODO
        // parser::schema::layers::Layer::Null(value) => {
        //     params = setup_layer(value, &mut layer);
        // }
        parser::schema::layers::AnyLayer::Precomposition(precomp_layer) => {
            params = setup_precomp_layer(precomp_layer, &mut layer);
            let name = precomp_layer.precomp_id.clone();
            let time_remap_in = precomp_layer
                .time_remap
                .as_ref()
                .unwrap_or(&FLOAT_VALUE_ZERO); // todo: verify that time remap should be 0 when none was parsed
            let time_remap = conv_scalar(time_remap_in);
            layer.content = Content::Instance { name, time_remap };
        }
        parser::schema::layers::AnyLayer::Shape(shape_layer) => {
            params = setup_shape_layer(shape_layer, &mut layer);
            let mut shapes = vec![];
            for shape in &shape_layer.shapes {
                if let Some(shape) = conv_shape(shape) {
                    shapes.push(shape);
                }
            }
            layer.content = Content::Shape(shapes);
        } //_ => {}, // todo: handle other todo shapes here
    }
    let (id, matte_mode) = params;
    Some((layer, id, matte_mode))
}

fn setup_precomp_layer(
    source: &parser::schema::layers::precomposition::PrecompositionLayer,
    target: &mut Layer,
) -> (usize, Option<BlendMode>) {
    target.name = source.properties.name.clone().unwrap_or_default();
    target.parent = source
        .properties
        .parent_index
        .as_ref()
        .map(|i| i.unwrap_u32() as usize);
    let (transform, opacity) = conv_transform(&source.properties.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.width = source.width.unwrap_u32();
    target.height = source.height.unwrap_u32();
    target.is_mask = source
        .properties
        .matte_target
        .as_ref()
        .map_or(false, |td| *td == BoolInt::True);

    let matte_mode =
        source
            .properties
            .matte_mode
            .as_ref()
            .map(|mode| match mode {
                schema::constants::matte_mode::MatteMode::Normal => {
                    Mix::Normal.into()
                }
                schema::constants::matte_mode::MatteMode::Alpha
                | schema::constants::matte_mode::MatteMode::Luma => {
                    Compose::SrcIn.into()
                }
                schema::constants::matte_mode::MatteMode::InvertedAlpha
                | schema::constants::matte_mode::MatteMode::InvertedLuma => {
                    Compose::SrcOut.into()
                }
            });

    target.blend_mode =
        conv_blend_mode(source.properties.blend_mode.as_ref().unwrap_or(
            &crate::parser::schema::constants::blend_mode::BlendMode::Normal,
        ));
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.frames = source.properties.in_point.unwrap_f32()
        ..source.properties.out_point.unwrap_f32();
    target.stretch = source
        .properties
        .time_stretch
        .as_ref()
        .map_or(0.0, |sr| sr.unwrap_f32());
    target.start_frame = source.properties.start_time.unwrap_f32();
    target.stretch = source
        .properties
        .time_stretch
        .as_ref()
        .map_or(0.0, |sr| sr.unwrap_f32());
    // todo: masks
    // for mask_source in &source.masks {
    //     if let Some(geometry) = conv_shape_geometry(&mask_source.points) {
    //         let mode = peniko::BlendMode::default();
    //         let opacity = conv_scalar(&mask_source.opacity);
    //         target.masks.push(Mask {
    //             mode,
    //             geometry,
    //             opacity,
    //         })
    //     }
    // }

    // todo: matte mode
    // let matte_mode = source.matte_mode.as_ref().map(|mode| match mode {
    //     MatteMode::Normal => Mix::Normal.into(),
    //     MatteMode::Alpha | MatteMode::Luma => Compose::SrcIn.into(),
    //     MatteMode::InvertAlpha | MatteMode::InvertLuma => {
    //         Compose::SrcOut.into()
    //     }
    // });

    (
        source
            .properties
            .index
            .as_ref()
            .map_or(0, |ind| ind.unwrap_u32()) as usize,
        matte_mode,
    )
}

fn setup_shape_layer(
    source: &parser::schema::layers::shape::ShapeLayer,
    target: &mut Layer,
) -> (usize, Option<BlendMode>) {
    target.name = source.properties.name.clone().unwrap_or_default();
    target.parent = source
        .properties
        .parent_index
        .as_ref()
        .map(|i| i.unwrap_u32() as usize);
    let (transform, opacity) = conv_transform(&source.properties.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.is_mask = source
        .properties
        .matte_target
        .as_ref()
        .map_or(false, |td| *td == BoolInt::True);

    let matte_mode =
        source
            .properties
            .matte_mode
            .as_ref()
            .map(|mode| match mode {
                schema::constants::matte_mode::MatteMode::Normal => {
                    Mix::Normal.into()
                }
                schema::constants::matte_mode::MatteMode::Alpha
                | schema::constants::matte_mode::MatteMode::Luma => {
                    Compose::SrcIn.into()
                }
                schema::constants::matte_mode::MatteMode::InvertedAlpha
                | schema::constants::matte_mode::MatteMode::InvertedLuma => {
                    Compose::SrcOut.into()
                }
            });

    target.blend_mode =
        conv_blend_mode(source.properties.blend_mode.as_ref().unwrap_or(
            &crate::parser::schema::constants::blend_mode::BlendMode::Normal,
        ));
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.frames = source.properties.in_point.unwrap_f32()
        ..source.properties.out_point.unwrap_f32();
    target.stretch = source
        .properties
        .time_stretch
        .as_ref()
        .map_or(0.0, |sr| sr.unwrap_f32());
    target.start_frame = source.properties.start_time.unwrap_f32();
    target.stretch = source
        .properties
        .time_stretch
        .as_ref()
        .map_or(0.0, |sr| sr.unwrap_f32());
    // todo: masks
    // for mask_source in &source.masks {
    //     if let Some(geometry) = conv_shape_geometry(&mask_source.points) {
    //         let mode = peniko::BlendMode::default();
    //         let opacity = conv_scalar(&mask_source.opacity);
    //         target.masks.push(Mask {
    //             mode,
    //             geometry,
    //             opacity,
    //         })
    //     }
    // }

    // todo: matte mode
    // let matte_mode = source.matte_mode.as_ref().map(|mode| match mode {
    //     MatteMode::Normal => Mix::Normal.into(),
    //     MatteMode::Alpha | MatteMode::Luma => Compose::SrcIn.into(),
    //     MatteMode::InvertAlpha | MatteMode::InvertLuma => {
    //         Compose::SrcOut.into()
    //     }
    // });

    (
        source
            .properties
            .index
            .as_ref()
            .map_or(0, |ind| ind.unwrap_u32()) as usize,
        matte_mode,
    )
}

fn conv_transform(
    value: &parser::schema::helpers::transform::Transform,
) -> (runtime::model::Transform, Value<f32>) {
    let rotation_in = match &value.rotation {
        Some(any_trans) => match any_trans {
            parser::schema::helpers::transform::AnyTransformR::Rotation(float_value) => float_value,
            // todo: need to actually handle split rotations
            parser::schema::helpers::transform::AnyTransformR::SplitRotation { .. } => todo!(),
        },
        None => todo!("split rotation"),
    };

    let position_in = match &value.position {
        schema::helpers::transform::AnyTransformP::Position(position) => {
            position
        }
        schema::helpers::transform::AnyTransformP::SplitPosition(_) => {
            // todo: split vectors
            todo!("split vector");
        }
    };

    let transform = animated::Transform {
        anchor: conv_point(
            value.anchor_point.as_ref().unwrap_or(&POSITION_ZERO),
        ),
        position: Position::Point(conv_point(position_in)), // todo
        scale: conv_vec2(value.scale.as_ref().unwrap_or(&MULTIDIM_ONE)),
        rotation: conv_scalar(rotation_in),
        skew: conv_scalar(value.skew.as_ref().unwrap_or(&FLOAT_VALUE_ZERO)),
        skew_angle: conv_scalar(
            value.skew_axis.as_ref().unwrap_or(&FLOAT_VALUE_ZERO),
        ),
    };
    let opacity =
        conv_scalar(value.opacity.as_ref().unwrap_or(&FLOAT_VALUE_ONE_HUNDRED));
    (transform.to_model(), opacity)
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
        anchor: conv_point(
            value
                .transform
                .anchor_point
                .as_ref()
                .unwrap_or(&POSITION_ZERO),
        ),
        position: Position::Point(conv_point(position_in)),
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

fn conv_scalar(
    float_value: &parser::schema::animated_properties::value::FloatValue,
) -> Value<f32> {
    use crate::parser::schema::animated_properties::animated_property::AnimatedPropertyK::*;
    match &float_value.animated_property.value {
        Static(number) => Value::Fixed(number.unwrap_f32()),
        AnimatedValue(keyframes) => {
            let mut frames = vec![];
            let mut values = vec![];
            for keyframe in keyframes {
                let start_time = keyframe.base.time.unwrap_f32();
                let data = keyframe.value[0].unwrap_f32();
                frames.push(crate::runtime::model::Time { frame: start_time });
                values.push(data);
                // todo: end_value deprecated but should we still push it if it exists?
            }
            Value::Animated(model::Animated {
                times: frames,
                values,
            })
        }
    }
}

fn conv_multi<T: Lerp>(
    multidimensional: &parser::schema::animated_properties::multi_dimensional::MultiDimensional,
    f: impl Fn(&Vec<f64>) -> T,
) -> Value<T> {
    use crate::parser::schema::animated_properties::animated_property::AnimatedPropertyK::*;

    match &multidimensional.animated_property.value {
        Static(components) => {
            let value: Vec<f64> = components
                .iter()
                .map(|number| number.as_f64().unwrap())
                .collect();
            Value::Fixed(f(&value))
        }
        AnimatedValue(keyframes) => {
            let mut frames = vec![];
            let mut values = vec![];
            for keyframe in keyframes {
                let data: Vec<f64> = keyframe
                    .value
                    .iter()
                    .map(|number| number.as_f64().unwrap())
                    .collect();
                frames.push(Time {
                    frame: keyframe.base.time.unwrap_f32(),
                });
                values.push(f(&data));
                // todo: end_value deprecated but should we still append it if it exists?
            }
            Value::Animated(model::Animated {
                times: frames,
                values,
            })
        }
    }
}

fn conv_multi_color<T: Lerp>(
    color: &parser::schema::animated_properties::color_value::ColorValue,
    f: impl Fn(&Vec<f64>) -> T,
) -> Value<T> {
    use crate::parser::schema::animated_properties::animated_property::AnimatedPropertyK::*;

    match &color.animated_property.value {
        Static(components) => {
            let value: Vec<f64> = components
                .iter()
                .map(|number| number.as_f64().unwrap())
                .collect();
            Value::Fixed(f(&value))
        }
        AnimatedValue(keyframes) => {
            let mut frames = vec![];
            let mut values = vec![];
            for keyframe in keyframes {
                let data: Vec<f64> = keyframe
                    .value
                    .iter()
                    .map(|number| number.as_f64().unwrap())
                    .collect();
                frames.push(Time {
                    frame: keyframe.base.time.unwrap_f32(),
                });
                values.push(f(&data));
                // todo: end_value deprecated but should we still append it if it exists?
            }
            Value::Animated(model::Animated {
                times: frames,
                values,
            })
        }
    }
}

fn conv_pos<T: Lerp>(
    position: &parser::schema::animated_properties::position::Position,
    f: impl Fn(&Vec<f64>) -> T,
) -> Value<T> {
    use crate::parser::schema::animated_properties::position::PositionValueK::*;

    match &position.value {
        Static(components) => {
            let value: Vec<f64> = components
                .iter()
                .map(|number| number.as_f64().unwrap())
                .collect();
            Value::Fixed(f(&value))
        }
        Animated(keyframes) => {
            let mut frames = vec![];
            let mut values = vec![];
            for keyframe in keyframes {
                let data: Vec<f64> = keyframe
                    .keyframe
                    .value
                    .iter()
                    .map(|number| number.as_f64().unwrap())
                    .collect();
                frames.push(Time {
                    frame: keyframe.keyframe.base.time.unwrap_f32(),
                });
                values.push(f(&data));
                // todo: end_value deprecated but should we still append it if it exists?
            }
            Value::Animated(model::Animated {
                times: frames,
                values,
            })
        }
    }
}

fn conv_point(
    value: &schema::animated_properties::position::Position,
) -> Value<Point> {
    conv_pos(value, |x| {
        Point::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

fn conv_color(
    value: &schema::animated_properties::color_value::ColorValue,
) -> Value<Color> {
    conv_multi_color(value, |x| {
        Color::rgb(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
            x.get(2).copied().unwrap_or(0.0),
        )
    })
}

fn conv_vec2(value: &MultiDimensional) -> Value<Vec2> {
    conv_multi(value, |x| {
        Vec2::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

fn conv_size(value: &MultiDimensional) -> Value<Size> {
    conv_multi(value, |x| {
        Size::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

// todo
// fn conv_gradient_colors(
//     value: &bodymovin::helpers::GradientColors,
// ) -> ColorStops {
//     use bodymovin::properties::Value::*;
//     let count = value.count as usize;
//     match &value.colors.value {
//         Fixed(value) => {
//             let mut stops = fixed::ColorStops::new();
//             for chunk in value.chunks_exact(4) {
//                 stops.push(
//                     (
//                         chunk[0] as f32,
//                         fixed::Color::rgba(chunk[1], chunk[2], chunk[3], 1.0),
//                     )
//                         .into(),
//                 )
//             }
//             ColorStops::Fixed(stops)
//         }
//         Animated(animated) => {
//             let mut frames = vec![];
//             let mut values = vec![];
//             let mut last_value = None;
//             for value in animated {
//                 if let Some(data) =
//                     value.start_value.as_ref().or(last_value.flatten())
//                 {
//                     frames.push(Time {
//                         frame: value.start_time as f32,
//                     });
//                     values.push(
//                         data.iter().map(|x| *x as f32).collect::<Vec<_>>(),
//                     );
//                 }
//                 last_value = Some(value.end_value.as_ref());
//             }
//             ColorStops::Animated(animated::ColorStops {
//                 frames,
//                 values,
//                 count,
//             })
//         }
//     }
// }

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
        // todo: gradients
        // Shape::GradientFill(value) => {
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
        //         stroke: None,
        //         brush,
        //         opacity: Value::Fixed(100.0),
        //     })
        // }
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
                position: conv_point(&value.position),
                size: conv_size(&value.size),
            };
            Some(crate::runtime::model::Geometry::Ellipse(ellipse))
        }
        AnyShape::Rectangle(value) => {
            let rect = animated::Rect {
                is_ccw: false, // todo: lottie schema does not have a field for this (anymore?)
                position: conv_point(&value.position),
                size: conv_size(&value.size),
                corner_radius: conv_scalar(&value.rounded_corner_radius),
            };
            Some(crate::runtime::model::Geometry::Rect(rect))
        }
        AnyShape::Path(value) => conv_shape_geometry(&value.shape),
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
    let is_closed = value
        .closed
        .as_ref()
        .unwrap_or(&BoolInt::False)
        .eq(&BoolInt::True);
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
