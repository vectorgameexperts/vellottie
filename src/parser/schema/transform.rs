//! Transform - https://lottiefiles.github.io/lottie-docs/concepts/#transform
use crate::parser::schema::animated_properties::{
    multi_dimensional::MultiDimensional, position::Position, value::Scalar,
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
    pub position: Position,
    /// Scale factor, 100 for no scaling
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<MultiDimensional>,
    /// Rotation in degrees, clockwise
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Scalar>,
    /// Skew amount as an angle in degrees
    #[serde(rename = "sk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew: Option<Scalar>,
    /// Direction at which skew is applied, in degrees (0 skews along the X
    /// axis, 90 along the Y axis)
    #[serde(rename = "sa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew_axis: Option<MultiDimensional>,
    /// Opacity, 100 for fully opaque
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<Scalar>,
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
        let position = obj
            .extract_obj(breadcrumb, "p")
            .and_then(|obj| Position::from_obj(breadcrumb, &obj))?;
        let scale = obj
            .extract_obj(breadcrumb, "s")
            .and_then(|obj| MultiDimensional::from_obj(breadcrumb, &obj))
            .ok();
        let rotation = obj
            .extract_obj(breadcrumb, "r")
            .and_then(|obj| Scalar::from_obj(breadcrumb, &obj))
            .ok();
        let skew = obj
            .extract_obj(breadcrumb, "sk")
            .and_then(|obj| Scalar::from_obj(breadcrumb, &obj))
            .ok();
        let skew_axis = obj
            .extract_obj(breadcrumb, "sa")
            .and_then(|obj| MultiDimensional::from_obj(breadcrumb, &obj))
            .ok();
        let opacity = obj
            .extract_obj(breadcrumb, "o")
            .and_then(|obj| Scalar::from_obj(breadcrumb, &obj))
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
