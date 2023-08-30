use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Colors are represented as arrays with values between 0 and 1 for the RGB components.

/// for example:

/// [1, 0, 0]
/// [1, 0.5, 0]
/// Note sometimes you might find color values with 4 components (the 4th being alpha) but most player ignore the last component.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Color {
    /// Color as a [r, g, b] array with values in Range [0, 1]
    #[serde(flatten)]
    color: Vec<Number>,
}
