use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Lottie {
    #[serde(rename = "v")]
    pub version: String,
}

impl Lottie {
    pub fn to_json(&self) -> serde_json::value::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Display for Lottie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
