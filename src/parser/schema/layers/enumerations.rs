use serde_repr::{Deserialize_repr, Serialize_repr};

/// Layer type
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum LayerType {
    Precomposition = 0,
    SolidColor = 1,
    Image = 2,
    Null = 3,
    Shape = 4,
    Text = 5,
    Audio = 6,
    VideoPlaceholder = 7,
    ImageSequence = 8,
    Video = 9,
    ImagePlaceholder = 10,
    Guide = 11,
    Adjustment = 12,
    Camera = 13,
    Light = 14,
    Data = 15,
}

/// How to stack copies in a repeater
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum Composite {
    Above = 1,
    Below = 2,
}

/// Boolean operation on shapes
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum MergeMode {
    Normal = 1,
    Add = 2,
    Subtract = 3,
    Intersect = 4,
    ExcludeIntersections = 5,
}

/// Star or Polygon
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum StarType {
    Star = 1,
    Polygon = 2,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextGrouping {
    Characters = 1,
    Words = 2,
    Lines = 3,
    All = 4,
}

/// Unit type for a text selector
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextRangeUnits {
    Percent = 1,
    Index = 2,
}
