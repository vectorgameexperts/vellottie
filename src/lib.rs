mod error;

pub mod breadcrumb;
pub(crate) mod util;

pub mod models;

pub use error::Error;
pub use models::lottie::Lottie;

pub fn from_bytes(b: &[u8]) -> Result<Lottie, Error> {
    let json_tree: serde_json::Value = serde_json::from_slice(b)?;
    from_json(&json_tree)
}

pub fn from_str(s: &str) -> Result<Lottie, Error> {
    let json_tree: serde_json::Value = serde_json::from_str(s)?;
    from_json(&json_tree)
}

pub fn from_json(v: &serde_json::Value) -> Result<Lottie, Error> {
    Lottie::from_json(v)
}
