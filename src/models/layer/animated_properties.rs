use super::BoolInt;
use crate::{
    breadcrumb::Breadcrumb,
    error::ValueType,
    util::{self},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use util::MapExt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VectorKeyframe {
    /// Keyframe time (in frames)
    #[serde(rename = "t")]
    pub time: Number,
    /// Value, note that scalar values have the value is wrapped in an array
    #[serde(rename = "s")]
    pub value: [Number; 2],
    /// Determines the curve as it enters the next keyframe
    #[serde(rename = "i")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing_in_handle: Option<()>, // TODO: EasingHandle
    /// Determines the curve as it exits the current keyframe
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing_out_handle: Option<()>, // TODO: EasingHandle
    /// Whether it's a hold frame
    #[serde(rename = "h", default)]
    #[serde(skip_serializing_if = "util::is_false")]
    pub hold_frame: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StaticVector {
    /// Whether the property is animated
    #[serde(rename = "a", default)]
    #[serde(skip_serializing_if = "util::is_false_int")]
    pub animated: BoolInt,
    #[serde(rename = "k")]
    /// An animated 2D vector number value
    pub value: [Number; 2],
    // TODO: ix field
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VectorKeyframes {
    /// Whether the property is animated
    #[serde(rename = "a", default)]
    #[serde(skip_serializing_if = "util::is_false_int")]
    pub animated: BoolInt,
    #[serde(rename = "k")]
    /// An animated 2D vector number value
    pub value: Vec<VectorKeyframe>,
    // TODO: ix field
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnimatedVector {
    Static(StaticVector),
    Animated(VectorKeyframes),
}

impl AnimatedVector {
    pub fn from_object(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        if animated == BoolInt::True {
            todo!();
        } else {
            Ok(AnimatedVector::Static(StaticVector {
                animated,
                value: obj.extract_type(breadcrumb, "k", ValueType::Scalar2d)?,
            }))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumberKeyframe {
    /// Keyframe time (in frames)
    #[serde(rename = "t")]
    pub time: Number,
    /// Value, note that scalar values have the value is wrapped in an array
    #[serde(rename = "s")]
    pub value: Number,
    /// Determines the curve as it enters the next keyframe
    #[serde(rename = "i")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing_in_handle: Option<()>, // TODO: EasingHandle
    /// Determines the curve as it exits the current keyframe
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing_out_handle: Option<()>, // TODO: EasingHandle
    /// Whether it's a hold frame
    #[serde(rename = "h", default)]
    #[serde(skip_serializing_if = "util::is_false_int")]
    pub hold_frame: BoolInt,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StaticNumberValue {
    #[serde(rename = "k")]
    /// An animated number value
    pub value: Number,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StaticNumber {
    #[serde(rename = "a")]
    pub animated: BoolInt,
    #[serde(rename = "k")]
    /// An animated number value
    pub value: Number,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumberKeyframes {
    #[serde(rename = "a")]
    pub animated: BoolInt,
    #[serde(rename = "k")]
    /// An animated number value
    pub value: Vec<NumberKeyframe>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnimatedNumber {
    Static(StaticNumber),
    Animated(NumberKeyframes),
}

impl AnimatedNumber {
    pub fn from_object(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        if animated == BoolInt::True {
            todo!();
        } else {
            Ok(AnimatedNumber::Static(StaticNumber {
                animated,
                value: obj.extract_number(breadcrumb, "k")?,
            }))
        }
    }
}
