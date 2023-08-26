mod error;
mod model;
pub(crate) mod util;

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

    let parent = "root";
    let version = root.extract_string(parent, "v")?;
    let frame_rate = root.extract_number(parent, "fr")?;
    let in_point = root.extract_number(parent, "ip")?;
    let out_point = root.extract_number(parent, "op")?;
    let width = root.extract_int(parent, "w")?;
    let height = root.extract_int(parent, "h")?;
    let three_dimensional = root.extract_bool(parent, "ddd")?;

    Ok(Lottie {
        version,
        frame_rate,
        in_point,
        out_point,
        width,
        height,
        three_dimensional,
    })
}
