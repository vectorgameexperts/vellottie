use crate::import::NumberExt;
use crate::parser;
use crate::parser::schema::animated_properties::multi_dimensional::MultiDimensional;
use crate::runtime::model::{self, Lerp, Time, Value};
use parser::schema;
use vello::kurbo::{Point, Size, Vec2};
use vello::peniko::Color;

pub fn conv_scalar(
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

pub fn conv_multi<T: Lerp>(
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

pub fn conv_multi_color<T: Lerp>(
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

pub fn conv_pos<T: Lerp>(
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

#[allow(clippy::get_first)]
pub fn conv_pos_point(
    value: &schema::animated_properties::position::Position,
) -> Value<Point> {
    conv_pos(value, |x| {
        Point::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

#[allow(clippy::get_first)]
pub fn conv_multi_point(
    value: &schema::animated_properties::multi_dimensional::MultiDimensional,
) -> Value<Point> {
    conv_multi(value, |x| {
        Point::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

#[allow(clippy::get_first)]
pub fn conv_color(
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

#[allow(clippy::get_first)]
pub fn conv_vec2(value: &MultiDimensional) -> Value<Vec2> {
    conv_multi(value, |x| {
        Vec2::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

#[allow(clippy::get_first)]
pub fn conv_size(value: &MultiDimensional) -> Value<Size> {
    conv_multi(value, |x| {
        Size::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}
