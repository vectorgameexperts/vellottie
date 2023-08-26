mod error;
mod model;
mod util;

use error::ValueType;
use util::MapExt;

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
        .ok_or(Error::IncorrectType("root".to_string(), ValueType::Map))?;

    let version = root.extract_string("root", "v")?;

    Ok(Lottie { version })
}
