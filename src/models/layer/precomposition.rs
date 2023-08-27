use super::{animated_properties::AnimatedNumber, common};
use crate::{breadcrumb::Breadcrumb, util::MapExt, Error};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrecompositionLayer {
    #[serde(flatten)]
    properties: common::LayerProperties,
    /// ID of the precomp as specified in the assets
    #[serde(rename = "refID")]
    pub precomp_id: String,
    /// Width of the clipping rect
    #[serde(rename = "w")]
    pub width: Number,
    /// Height of the clipping rect
    #[serde(rename = "h")]
    pub height: Number,
    /// Time Remapping
    #[serde(rename = "tm")]
    pub time_remap: AnimatedNumber,
}

impl PrecompositionLayer {
    pub fn from_properties_and_object(
        breadcrumb: &mut Breadcrumb,
        properties: common::LayerProperties,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let precomp_id = obj.extract_string(breadcrumb, "refID")?;
        let width = obj.extract_number(breadcrumb, "w")?;
        let height = obj.extract_number(breadcrumb, "h")?;
        let time_remap = AnimatedNumber::from_object(breadcrumb, obj)?;

        Ok(PrecompositionLayer {
            properties,
            precomp_id,
            width,
            height,
            time_remap,
        })
    }
}
