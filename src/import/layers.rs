use crate::parser;
use crate::parser::schema::animated_properties::split_vector::SplitVector;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use crate::runtime;
use crate::runtime::model::animated::Position;
use crate::runtime::model::{animated, Content, Layer, Value};
use parser::schema;
use vello::peniko::{self, BlendMode, Compose, Mix};

use super::defaults::{
    FLOAT_VALUE_ONE_HUNDRED, FLOAT_VALUE_ZERO, MULTIDIM_ONE, POSITION_ZERO,
};
use super::properties::{conv_pos_point, conv_scalar, conv_vec2};
use super::{conv_blend_mode, conv_shape, conv_shape_geometry, NumberExt};

pub fn conv_layer(
    source: &parser::schema::layers::AnyLayer,
) -> Option<(Layer, usize, Option<BlendMode>)> {
    let mut layer = Layer::default();

    let params = match source {
        parser::schema::layers::AnyLayer::Null(null_layer) => {
            if let Some(true) = null_layer.properties.hidden {
                return None;
            }

            setup_layer_base(&null_layer.properties, &mut layer)
        }
        parser::schema::layers::AnyLayer::Precomposition(precomp_layer) => {
            if let Some(true) = precomp_layer.properties.hidden {
                return None;
            }

            let params = setup_precomp_layer(precomp_layer, &mut layer);
            let name = precomp_layer.precomp_id.clone();
            let time_remap_in = precomp_layer
                .time_remap
                .as_ref()
                .unwrap_or(&FLOAT_VALUE_ZERO); // todo: verify that time remap should be 0 when none was parsed
            let time_remap = conv_scalar(time_remap_in);
            layer.content = Content::Instance { name, time_remap };

            params
        }
        parser::schema::layers::AnyLayer::Shape(shape_layer) => {
            if let Some(true) = shape_layer.properties.hidden {
                return None;
            }

            let params = setup_shape_layer(shape_layer, &mut layer);
            let mut shapes = vec![];
            for shape in &shape_layer.shapes {
                if let Some(shape) = conv_shape(shape) {
                    shapes.push(shape);
                }
            }
            layer.content = Content::Shape(shapes);

            params
        }
        schema::layers::AnyLayer::SolidColor(solid_color_layer) => {
            if let Some(true) = solid_color_layer.properties.hidden {
                return None;
            }

            setup_layer_base(&solid_color_layer.properties, &mut layer)
        }
    };

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

    for mask_source in source
        .properties
        .masks_properties
        .as_ref()
        .unwrap_or(&Vec::default())
    {
        if let Some(shape) = &mask_source.shape {
            if let Some(geometry) = conv_shape_geometry(shape) {
                let mode = peniko::BlendMode::default();
                let opacity = conv_scalar(
                    mask_source
                        .opacity
                        .as_ref()
                        .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
                );
                target.masks.push(runtime::model::Mask {
                    mode,
                    geometry,
                    opacity,
                })
            }
        }
    }

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

    for mask_source in source
        .properties
        .masks_properties
        .as_ref()
        .unwrap_or(&Vec::default())
    {
        if let Some(shape) = &mask_source.shape {
            if let Some(geometry) = conv_shape_geometry(shape) {
                let mode = peniko::BlendMode::default();
                let opacity = conv_scalar(
                    mask_source
                        .opacity
                        .as_ref()
                        .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
                );
                target.masks.push(runtime::model::Mask {
                    mode,
                    geometry,
                    opacity,
                })
            }
        }
    }

    (
        source
            .properties
            .index
            .as_ref()
            .map_or(0, |ind| ind.unwrap_u32()) as usize,
        matte_mode,
    )
}

fn setup_layer_base(
    source: &parser::schema::layers::visual::VisualLayer,
    target: &mut Layer,
) -> (usize, Option<BlendMode>) {
    target.name = source.name.clone().unwrap_or_default();
    target.parent = source
        .parent_index
        .as_ref()
        .map(|i| i.unwrap_u32() as usize);
    let (transform, opacity) = conv_transform(&source.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.is_mask = source
        .matte_target
        .as_ref()
        .map_or(false, |td| *td == BoolInt::True);

    let matte_mode = source.matte_mode.as_ref().map(|mode| match mode {
        schema::constants::matte_mode::MatteMode::Normal => Mix::Normal.into(),
        schema::constants::matte_mode::MatteMode::Alpha
        | schema::constants::matte_mode::MatteMode::Luma => {
            Compose::SrcIn.into()
        }
        schema::constants::matte_mode::MatteMode::InvertedAlpha
        | schema::constants::matte_mode::MatteMode::InvertedLuma => {
            Compose::SrcOut.into()
        }
    });

    target.blend_mode = conv_blend_mode(source.blend_mode.as_ref().unwrap_or(
        &crate::parser::schema::constants::blend_mode::BlendMode::Normal,
    ));
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.frames = source.in_point.unwrap_f32()..source.out_point.unwrap_f32();
    target.stretch = source
        .time_stretch
        .as_ref()
        .map_or(0.0, |sr| sr.unwrap_f32());
    target.start_frame = source.start_time.unwrap_f32();
    target.stretch = source
        .time_stretch
        .as_ref()
        .map_or(0.0, |sr| sr.unwrap_f32());

    for mask_source in
        source.masks_properties.as_ref().unwrap_or(&Vec::default())
    {
        if let Some(shape) = &mask_source.shape {
            if let Some(geometry) = conv_shape_geometry(shape) {
                let mode = peniko::BlendMode::default();
                let opacity = conv_scalar(
                    mask_source
                        .opacity
                        .as_ref()
                        .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
                );
                target.masks.push(runtime::model::Mask {
                    mode,
                    geometry,
                    opacity,
                })
            }
        }
    }

    (
        source.index.as_ref().map_or(0, |ind| ind.unwrap_u32()) as usize,
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

    let position = match &value.position {
        schema::helpers::transform::AnyTransformP::Position(position) => {
            Position::Value(conv_pos_point(position))
        }
        schema::helpers::transform::AnyTransformP::SplitPosition(
            SplitVector { x, y, .. },
        ) => Position::SplitValues((conv_scalar(x), conv_scalar(y))),
    };

    let transform = animated::Transform {
        anchor: conv_pos_point(
            value.anchor_point.as_ref().unwrap_or(&POSITION_ZERO),
        ),
        position,
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
