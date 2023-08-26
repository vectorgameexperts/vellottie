#[derive(Debug)]
pub enum ValueType {
    Null,
    Bool,
    Object,
    Map,
    Number,
    String,
    Array,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the file is not valid json")]
    NotJson(#[from] serde_json::Error),

    #[error("the parent object '{0}' expected the child key '{1}'")]
    MissingChild(String, String),

    #[error("{0} is the wrong type, expected a {1:?}")]
    IncorrectType(String, ValueType),
}
