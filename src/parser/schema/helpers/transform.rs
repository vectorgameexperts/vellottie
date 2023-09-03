//! Transform - https://lottiefiles.github.io/lottie-docs/concepts/#transform
use crate::parser::breadcrumb::ValueType;
use crate::parser::schema::animated_properties::split_vector::SplitVector;
use crate::parser::schema::animated_properties::{
    multi_dimensional::MultiDimensional, position::Position, value::FloatValue,
};
use crate::parser::{breadcrumb::Breadcrumb, error::Error, util::MapExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transform {
    /// Position (relative to its parent) around which transformations are
    /// applied (ie: center for rotation / scale)
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_point: Option<Position>,
    /// Position / Translation
    #[serde(rename = "p")]
    pub position: AnyTransformP,
    /// Scale factor, 100 for no scaling
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<MultiDimensional>,
    /// Rotation in degrees, clockwise
    // todo: need untagged enum for split vector variant
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<AnyTransformR>,
    /// Skew amount as an angle in degrees
    #[serde(rename = "sk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew: Option<FloatValue>,
    /// Direction at which skew is applied, in degrees (0 skews along the X
    /// axis, 90 along the Y axis)
    #[serde(rename = "sa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew_axis: Option<FloatValue>,
    /// Opacity, 100 for fully opaque
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,
}

impl Transform {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let anchor_point = obj
            .extract_obj(breadcrumb, "a")
            .and_then(|obj| Position::from_obj(breadcrumb, &obj))
            .ok();
        let position = obj.extract_obj(breadcrumb, "p").and_then(|obj| {
            if obj.contains_key("s") {
                Ok(AnyTransformP::SplitPosition(SplitVector::from_obj(
                    breadcrumb, &obj,
                )?))
            } else {
                Ok(AnyTransformP::Position(Position::from_obj(
                    breadcrumb, &obj,
                )?))
            }
        })?;
        let scale = obj
            .extract_obj(breadcrumb, "s")
            .and_then(|obj| MultiDimensional::from_obj(breadcrumb, &obj))
            .ok();
        let rotation = AnyTransformR::from_obj(breadcrumb, obj).ok();
        let skew = obj
            .extract_obj(breadcrumb, "sk")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();
        let skew_axis = obj
            .extract_obj(breadcrumb, "sa")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();
        let opacity = obj
            .extract_obj(breadcrumb, "o")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok();
        let transform = Transform {
            anchor_point,
            position,
            scale,
            rotation,
            skew,
            skew_axis,
            opacity,
        };
        Ok(transform)
    }
}

/// The possible values of "p" in a [`Transform`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnyTransformP {
    /// Position / Translation
    Position(Position),
    /// Position / Translation with split components
    SplitPosition(SplitVector),
}

/// The possible values of "r" in a [`Transform`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnyTransformR {
    /// Rotation in degrees, clockwise
    Rotation(FloatValue),
    /// Split rotation components
    SplitRotation {
        /// Split rotation X component.
        #[serde(rename = "x")]
        x_rotation: FloatValue,
        /// Split rotation Y component.
        #[serde(rename = "y")]
        y_rotation: FloatValue,
        /// Split rotation component, equivalent to r when not split.
        #[serde(rename = "z")]
        z_rotation: FloatValue,
        /// Orientation
        #[serde(rename = "or")]
        orientation: MultiDimensional,
    },
}

impl AnyTransformR {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::Rotation);
        let rotation = if let Some(rotation) = obj
            .extract_obj(breadcrumb, "r")
            .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
            .ok()
            .map(AnyTransformR::Rotation)
        {
            rotation
        } else {
            let x_rotation = obj
                .extract_obj(breadcrumb, "rx")
                .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?;
            let y_rotation = obj
                .extract_obj(breadcrumb, "ry")
                .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?;
            let z_rotation = obj
                .extract_obj(breadcrumb, "rz")
                .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))?;
            let orientation = obj
                .extract_obj(breadcrumb, "or")
                .and_then(|obj| MultiDimensional::from_obj(breadcrumb, &obj))?;
            // Split should always have the value "1", otherwise it is not a split vector object
            AnyTransformR::SplitRotation {
                x_rotation,
                y_rotation,
                z_rotation,
                orientation,
            }
        };

        breadcrumb.exit();
        Ok(rotation)
    }
}
