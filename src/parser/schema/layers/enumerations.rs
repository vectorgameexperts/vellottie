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

/// Layer and shape blend mode
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum BlendMode {
    Normal = 0,
    Multiply = 1,
    Screen = 2,
    Overlay = 3,
    Darken = 4,
    Lighten = 5,
    ColorDodge = 6,
    ColorBurn = 7,
    HardLight = 8,
    SoftLight = 9,
    Difference = 10,
    Exclusion = 11,
    Hue = 12,
    Saturation = 13,
    Color = 14,
    Luminosity = 15,
    Add = 16,
    HardMix = 17,
}

/// How to stack copies in a repeater
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum Composite {
    Above = 1,
    Below = 2,
}

/// Type of a gradient
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum GradientType {
    Linear = 1,
    Radial = 2,
}

/// How a layer should mask another layer
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum MatteMode {
    Normal = 0,
    Alpha = 1,
    InvertedAlpha = 2,
    Luma = 3,
    InvertedLuma = 4,
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

/// Drawing direction of the shape curve, useful for trim path
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum ShapeDirection {
    /// Usually clockwise
    Normal = 1,
    /// Usually counter clockwise
    Reversed = 3,
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
pub enum TextedBased {
    Characters = 1,
    CharacterExcludingSpaces = 2,
    Words = 3,
    Lines = 4,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextGrouping {
    Characters = 1,
    Words = 2,
    Lines = 3,
    All = 4,
}

/// Text capitalization
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextCaps {
    Regular = 0,
    AllCaps = 1,
    SmallCaps = 2,
}

/// Unit type for a text selector
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextRangeUnits {
    Percent = 1,
    Index = 2,
}
