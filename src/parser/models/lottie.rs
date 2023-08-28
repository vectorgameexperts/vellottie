use super::{assets::Asset, layer::Layer, BoolInt};
use crate::parser::{
    breadcrumb::Breadcrumb, error::ValueType, util::MapExt, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Lottie {
    /// Lottie file version
    #[serde(rename = "v")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    // Name, as seen from editors and the like
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Framerate in frames per second
    #[serde(rename = "fr")]
    pub frame_rate: Number,
    /// "In Point", which frame the animation starts at (usually 0)
    #[serde(rename = "ip")]
    pub in_point: Number,
    /// "Out Point", which frame the animation stops/loops at, which makes this
    /// the duration in frames when `ip` is 0
    #[serde(rename = "op")]
    pub out_point: Number,
    /// Width of the animation
    #[serde(rename = "w")]
    pub width: Number,
    /// Height of the animation
    #[serde(rename = "h")]
    pub height: Number,
    /// Whether the animation has 3D layers
    #[serde(rename = "ddd", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_dimensional: Option<BoolInt>,
    /// List of assets that can be referenced by layers
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
    /// List of layers
    #[serde(default)]
    pub layers: Vec<Layer>,
}

impl Lottie {
    pub fn to_json(&self) -> serde_json::value::Value {
        serde_json::to_value(self).unwrap()
    }

    pub fn from_json(v: &serde_json::Value) -> Result<Lottie, Error> {
        let root = v.as_object().ok_or(Error::FileNotObject)?;
        let mut breadcrumb = Breadcrumb::new();

        let name = root.extract_string(&breadcrumb, "nm").ok();
        if let Some(ref name) = name {
            breadcrumb.rename_root(name.clone());
        }
        let version = root.extract_string(&breadcrumb, "v").ok();
        let frame_rate = root.extract_number(&breadcrumb, "fr")?;
        let in_point = root.extract_number(&breadcrumb, "ip")?;
        let out_point = root.extract_number(&breadcrumb, "op")?;
        let width = root.extract_number(&breadcrumb, "w")?;
        let height = root.extract_number(&breadcrumb, "h")?;
        let three_dimensional = root.extract_bool_int(&breadcrumb, "ddd").ok();

        // Assets
        let mut assets = vec![];
        let json_assets =
            root.extract_arr(&breadcrumb, "assets").unwrap_or_default();
        breadcrumb.enter(ValueType::Array, Some("assets"));
        for v in json_assets {
            let asset = Asset::from_json(&mut breadcrumb, &v)?;
            assets.push(asset);
        }
        breadcrumb.exit();
        let assets = if assets.is_empty() {
            None
        } else {
            Some(assets)
        };

        // Layers
        let mut layers = vec![];
        let json_layers = root.extract_arr(&breadcrumb, "layers")?;
        breadcrumb.enter(ValueType::Array, Some("layers"));
        for v in json_layers {
            let layer = Layer::from_json(&mut breadcrumb, &v)?;
            layers.push(layer);
        }
        breadcrumb.exit();

        // Layers
        Ok(Lottie {
            version,
            name,
            frame_rate,
            in_point,
            out_point,
            width,
            height,
            three_dimensional,
            assets,
            layers,
        })
    }
}

impl Display for Lottie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
