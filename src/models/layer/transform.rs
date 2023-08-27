//! Transform - https://lottiefiles.github.io/lottie-docs/concepts/#transform

use super::animated_properties::{AnimatedNumber, AnimatedVector};
use crate::util::MapExt;
use crate::{breadcrumb::Breadcrumb, error::Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Transform {
    /// Position (relative to its parent) around which transformations are applied (ie: center for rotation / scale)
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_point: Option<AnimatedVector>,
    /// Position / Translation
    #[serde(rename = "p")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<AnimatedVector>,
    /// Scale factor, 100 for no scaling
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<AnimatedVector>,
    /// Rotation in degrees, clockwise
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<AnimatedNumber>,
    /// Skew amount as an angle in degrees
    #[serde(rename = "sk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew: Option<AnimatedNumber>,
    /// Direction at which skew is applied, in degrees (0 skews along the X axis, 90 along the Y axis)
    #[serde(rename = "sa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew_axis: Option<AnimatedNumber>,
    /// Opacity, 100 for fully opaque
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<AnimatedNumber>,
}

impl Transform {
    pub fn from_object(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter("ks");
        let anchor_point =
            AnimatedVector::from_object(breadcrumb, &obj.extract_obj(breadcrumb, "a")?).ok();
        let position =
            AnimatedVector::from_object(breadcrumb, &obj.extract_obj(breadcrumb, "p")?).ok();
        let scale =
            AnimatedVector::from_object(breadcrumb, &obj.extract_obj(breadcrumb, "s")?).ok();
        let rotation =
            AnimatedNumber::from_object(breadcrumb, &obj.extract_obj(breadcrumb, "r")?).ok();
        let skew =
            AnimatedNumber::from_object(breadcrumb, &obj.extract_obj(breadcrumb, "sk")?).ok();
        let skew_axis =
            AnimatedNumber::from_object(breadcrumb, &obj.extract_obj(breadcrumb, "sa")?).ok();
        let opacity =
            AnimatedNumber::from_object(breadcrumb, &obj.extract_obj(breadcrumb, "o")?).ok();
        let transform = Transform {
            anchor_point,
            position,
            scale,
            rotation,
            skew,
            skew_axis,
            opacity,
        };
        breadcrumb.exit();

        Ok(transform)
    }
}