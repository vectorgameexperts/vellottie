use super::LayerProperties;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NullLayer {
    #[serde(flatten)]
    pub properties: LayerProperties,
}
