mod error;
mod model;
mod util;

use error::ValueType;

pub use error::Error;
pub use model::Lottie;

pub fn from_bytes(b: &[u8]) -> Result<Lottie, Error> {
    let json_tree: serde_json::Value = serde_json::from_slice(b)?;
    from_json(&json_tree)
}

pub fn from_str(s: &str) -> Result<Lottie, Error> {
    let json_tree: serde_json::Value = serde_json::from_str(s)?;
    from_json(&json_tree)
}

pub fn from_json(v: &serde_json::Value) -> Result<Lottie, Error> {
    let root = v
        .as_object()
        .ok_or(Error::IncorrectType("root", ValueType::Map))?;

    //let version = root.extract_string("v")?;
    let version = root
        .get("v")
        .ok_or(Error::MissingChild("root".to_string(), "v".to_string()))?
        .as_str()
        .ok_or(Error::IncorrectType("v", ValueType::String))?
        .to_owned();

    Ok(Lottie { version })
}
