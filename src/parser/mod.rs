mod error;

pub mod breadcrumb;
pub(crate) mod util;

pub mod models;

pub use error::Error;
pub use models::lottie::Lottie;

pub fn from_slice(b: &[u8]) -> Result<Lottie, Error> {
    let json_tree: serde_json::Value = serde_json::from_slice(b)?;
    from_json(&json_tree)
}

pub fn from_str(s: &str) -> Result<Lottie, Error> {
    let json_tree: serde_json::Value =
        serde_json::from_str(s).map_err(|e| Error::FileNotJson(Box::new(e)))?;
    from_json(&json_tree)
}

pub fn from_json(v: &serde_json::Value) -> Result<Lottie, Error> {
    Lottie::from_json(v)
}

pub fn from_serde_slice(v: &[u8]) -> Result<Lottie, serde_json::Error> {
    serde_json::from_slice(v)
}

pub fn from_serde_str(s: &str) -> Result<Lottie, serde_json::Error> {
    serde_json::from_str(s)
}

pub fn from_serde_json(
    v: serde_json::Value,
) -> Result<Lottie, serde_json::Error> {
    serde_json::from_value(v)
}
