use crate::breadcrumb::Breadcrumb;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
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

    Lottie,
    Asset,
    Image,
    Precomposition,
    Layer,
    Shape,
    Transform,
    AnimatedVector,
    StaticVector,
    AnimatedNumber,
    StaticNumber,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValueType::Null => "null",
                ValueType::Bool => "boolean",
                ValueType::Object => "object",
                ValueType::Number => "number",
                ValueType::BoolInt => "0 or 1",
                ValueType::EnumInt => "0 to 255",
                ValueType::String => "string",
                ValueType::Array => "array",
                ValueType::Scalar2d => "[number, number]",
                ValueType::Asset => "Asset",
                ValueType::Layer => "Layer",
                ValueType::Shape => "Shape",
                ValueType::Lottie => "Lottie",
                ValueType::Image => "Image",
                ValueType::Precomposition => "Precomposition",
                ValueType::Transform => "Transform",
                ValueType::AnimatedVector => "AnimatedVector",
                ValueType::StaticVector => "StaticVector",
                ValueType::AnimatedNumber => "AnimatedNumber",
                ValueType::StaticNumber => "StaticNumber",
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
