use super::asset::Asset;
use crate::parser::schema::animation::composition::Composition;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use crate::parser::{
    breadcrumb::Breadcrumb, breadcrumb::ValueType, util::MapExt, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Asset containing an animation that can be referenced by layers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Precomposition {
    #[serde(flatten)]
    pub asset: Asset,
    #[serde(flatten)]
    pub composition: Composition,
    /// Framerate in frames per second
    #[serde(rename = "fr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<Number>,
    /// Extra composition
    #[serde(rename = "xt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<BoolInt>,
}

impl Precomposition {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let id = obj.extract_string(breadcrumb, "id").ok();
        breadcrumb.enter(ValueType::Precomposition, id);

        // Extract
        let asset = Asset::from_obj(breadcrumb, obj)?;
        let composition = Composition::from_obj(breadcrumb, obj)?;
        let frame_rate = obj.extract_number(breadcrumb, "fr").ok();
        let extra = obj.extract_bool_int(breadcrumb, "xt").ok();

        breadcrumb.exit();
        Ok(Self {
            asset,
            composition,
            frame_rate,
            extra,
        })
    }
}
