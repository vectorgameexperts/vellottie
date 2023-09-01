use super::multi_dimensional::MultiDimensional;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Represents colors and offsets in a gradient.
///
/// Colors are represented as a flat list interleaving offsets and color components.
/// There are two possible layouts:
/// - Without alpha, the colors are a sequence of offset, r, g, b
/// - With alpha, same as above but at the end of the list there is a sequence of offset, alpha
#[derive(Debug, Serialize, Deserialize)]
struct GradientColors {
    /// Colors in the gradient.
    #[serde(rename = "k")]
    colors: MultiDimensional,
    /// Number of colors in k.
    #[serde(rename = "p")]
    count: Number,
}

impl GradientColors {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::GradientColors);
        let colors = obj
            .extract_obj(breadcrumb, "k")
            .and_then(|obj| MultiDimensional::from_obj(breadcrumb, &obj))?;
        let count = obj.extract_number(breadcrumb, "p")?;
        breadcrumb.exit();
        Ok(GradientColors { colors, count })
    }
}
