use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Style at the end of a stoked line
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum LineCap {
    Butt = 1,
    Round = 2,
    Square = 3,
}

/// Style at a sharp corner of a stoked line
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum LineJoin {
    Miter = 1,
    Round = 2,
    Bevel = 3,
}

/// Type of a dash item in a stroked line
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StrokeDash {
    #[serde(rename = "d")]
    Dash,
    #[serde(rename = "g")]
    Gap,
    #[serde(rename = "o")]
    Offset,
}
