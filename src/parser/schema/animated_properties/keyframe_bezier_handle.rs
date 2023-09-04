use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Represents a keyframe bezier handle.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyframeBezierHandle {
    /// X-coordinate of the handle.
    /// - 0 means start time of the keyframe.
    /// - 1 means time of the next keyframe.
    #[serde(rename = "x")]
    pub x_coordinate: KeyframeComponent,
    /// Y-coordinate of the handle.
    /// - 0 means start value of the keyframe.
    /// - 1 means value at the next keyframe.
    #[serde(rename = "y")]
    pub y_coordinate: KeyframeComponent,
}

/// Represents a component of the keyframe.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum KeyframeComponent {
    /// Array of component values.
    ArrayOfValues(Vec<Number>),
    /// Single component value.
    SingleValue(Number),
}

impl KeyframeBezierHandle {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let x_coordinate =
            obj.extract_type(breadcrumb, "x", ValueType::Number)?;
        let y_coordinate =
            obj.extract_type(breadcrumb, "y", ValueType::Number)?;

        Ok(Self {
            x_coordinate,
            y_coordinate,
        })
    }
}
