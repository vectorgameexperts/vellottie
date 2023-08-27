use crate::breadcrumb::Breadcrumb;
use std::fmt::Display;

#[derive(Debug)]
pub enum ValueType {
    Null,
    Bool,
    Object,
    Number,
    BoolInt,
    EnumInt,
    String,
    Array,
    Scalar2d,

    // Named
    Asset,
    Layer,
    Shape,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ValueType::Null => "null",
                ValueType::Bool => "boolean",
                ValueType::Object => "object",
                ValueType::Number => "number",
                ValueType::BoolInt => "0 or 1",
                ValueType::EnumInt => "0 to 255",
                ValueType::String => "string",
                ValueType::Array => "array",
                ValueType::Scalar2d => "[number, number]",

                ValueType::Asset => "Image or Precomposition",
                ValueType::Layer => "array of Precomposition Layer or Solid Color Layer or Image Layer or Null Layer or Shape Layer or Text Layer or Audio Layer or Camera Layer or Data Layer",
                ValueType::Shape => "Rectangle or Ellipse or PolyStar or Path or Fill or Stroke or Gradient Fill or Gradient Stroke or No Style or Group or Transform or Repeater or Trim or Rounded Corners or Pucker / Bloat or Merge or Twist or Offset Path or Zig Zag"
            }
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the file is not valid json: {0:?}")]
    FileNotJson(Box<serde_json::Error>),

    #[error("encountered invalid json: {0:?}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("the root is not an object (e.g. {{\"a\": ...}})")]
    FileNotObject,

    #[error("expected the child key '{key}' in path: {breadcrumb}")]
    MissingChild { breadcrumb: Breadcrumb, key: String },

    #[error("expected children to be '{expected}' in path {breadcrumb}")]
    UnexpectedChild {
        breadcrumb: Breadcrumb,
        expected: ValueType,
    },

    #[error("'{key}' is the wrong type, expected: {expected}")]
    IncorrectType { key: String, expected: ValueType },
}
