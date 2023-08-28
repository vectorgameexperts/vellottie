use super::breadcrumb::ValueType;
use crate::parser::breadcrumb::Breadcrumb;

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
