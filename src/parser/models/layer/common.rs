#![allow(deprecated)]

use super::{
    enumerations::{BlendMode, LayerType},
    BoolInt, Transform,
};
use crate::parser::{breadcrumb::Breadcrumb, error::ValueType, util::MapExt, Error};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Common properties between layers
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayerProperties {
    /// Name, as seen from editors and the like
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Match name, used in expressions
    #[serde(rename = "mn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_name: Option<String>,
    /// Whether the layer is 3D. Lottie doesn't actually support 3D stuff so this should always be 0
    #[serde(rename = "ddd", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_dimensional: Option<BoolInt>,
    /// Whether the layer is hidden
    #[serde(rename = "hd", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Layer type, must be one of the values seen above
    #[serde(rename = "ty")]
    pub layer_type: LayerType,
    /// Layer index for parenting
    #[serde(rename = "ind")]
    pub index: Option<Number>,
    /// Parent index for parenting
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_index: Option<Number>,
    /// Time Stretch
    #[serde(rename = "sr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stretch: Option<Number>,
    /// Frame when the layer becomes visible
    #[serde(rename = "ip")]
    pub in_point: Number,
    /// Frame when the layer becomes invisible
    #[serde(rename = "op")]
    pub out_point: Number,
    /// Start Time
    #[serde(rename = "st")]
    pub start_time: Number,
    /// Matte mode
    #[serde(rename = "tt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matte_mode: Option<()>, // TODO: Matte Mode
    /// Matte target
    #[serde(rename = "td", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matte_target: Option<BoolInt>,
    /// Masks for the layer
    #[serde(rename = "maskProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_properties: Option<()>, //  TODO: array of Mask
    /// Effects for the layer
    #[serde(rename = "ef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<()>, // TODO: array of Custom Effect or Drop Shadow Effect or Fill Effect or Gaussian Blur Effect or Set Matte Effect or Pro Levels Effect or Stroke Effect or Tint Effect or Tritone Effect or Radial Wipe or Wavy Effect or Puppet Effect or Spherize Effect or Mesh Warp Effect or Displacement Map Effect or Twirl Effect
    /// Layer styles
    #[serde(rename = "sy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<()>, // TODO: array of Layer Stroke or Drop Shadow or Inner Shadow or Outer Glow or Inner Glow or Bevel Emboss or Satin or Color Overlay or Gradient Overlay
    /// Layer transform
    #[serde(rename = "ks")]
    pub transform: Transform,
    /// If 1, The layer will rotate itself to match its animated position path
    #[serde(rename = "ao", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_to_match_anim_pos_path: Option<BoolInt>,
    /// Index of the layer used as matte, if omitted assume the layer above the current one
    #[serde(rename = "tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matte_layer_index: Option<Number>,
    /// Whether the layer has masks applied
    #[serde(rename = "hasMask", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_mask: Option<bool>,
    /// Whether motion blur is enabled for the layer
    #[serde(rename = "mb", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_blur: Option<bool>,
    /// Blend Mode
    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    /// CSS class used by the SVG renderer
    #[serde(rename = "cl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,
    /// id attribute used by the SVG renderer
    #[serde(rename = "ln")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Tag name used by the SVG renderer
    #[serde(rename = "tg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    /// This is deprecated in favour of transform_before_mask
    #[deprecated(note = "please use `transform_before_mask` instead")]
    #[serde(rename = "cp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tranform_before_mask_deprecated: Option<String>,
    /// Marks that transforms should be applied before masks
    #[serde(rename = "ct", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_before_mask: Option<BoolInt>,
}

impl LayerProperties {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let name = obj.extract_string(breadcrumb, "nm").ok();
        let match_name = obj.extract_string(breadcrumb, "mn").ok();
        let three_dimensional = obj.extract_bool_int(breadcrumb, "ddd").ok();
        let hidden = obj.extract_bool(breadcrumb, "hd").ok();
        let layer_type: LayerType = obj.extract_type(breadcrumb, "ty", ValueType::EnumInt)?;
        let index = obj.extract_number(breadcrumb, "ind").ok();
        let parent_index = obj.extract_number(breadcrumb, "parent").ok();
        let time_stretch = obj.extract_number(breadcrumb, "sr").ok();
        let in_point = obj.extract_number(breadcrumb, "ip")?;
        let out_point = obj.extract_number(breadcrumb, "op")?;
        let start_time = obj.extract_number(breadcrumb, "st")?;
        let matte_target = obj.extract_bool_int(breadcrumb, "td").ok();
        let rotate_to_match_anim_pos_path = obj.extract_bool_int(breadcrumb, "ao").ok();
        let matte_layer_index = obj.extract_number(breadcrumb, "tp").ok();
        let has_mask = obj.extract_bool(breadcrumb, "hasmask").ok();
        let motion_blur = obj.extract_bool(breadcrumb, "mb").ok();
        let css_class = obj.extract_string(breadcrumb, "cl").ok();
        let id = obj.extract_string(breadcrumb, "ln").ok();
        let tag_name = obj.extract_string(breadcrumb, "tg").ok();
        let tranform_before_mask_deprecated = obj.extract_string(breadcrumb, "cp").ok();
        let transform_before_mask = obj.extract_bool_int(breadcrumb, "ct").ok();
        let transform = {
            let obj = obj.extract_obj(breadcrumb, "ks")?;
            breadcrumb.enter(ValueType::Transform, Some("ks"));
            let transform = Transform::from_obj(breadcrumb, &obj)?;
            breadcrumb.exit();
            transform
        };
        let blend_mode: Option<BlendMode> =
            obj.extract_type(breadcrumb, "bm", ValueType::EnumInt).ok();
        Ok(LayerProperties {
            name,
            match_name,
            three_dimensional,
            hidden,
            layer_type: layer_type.clone(),
            index,
            parent_index,
            time_stretch,
            in_point,
            out_point,
            start_time,
            matte_mode: None, // TODO
            matte_target,
            mask_properties: None, // TODO
            effects: None,         // TODO
            styles: None,          // TODO
            transform,
            rotate_to_match_anim_pos_path,
            matte_layer_index,
            has_mask,
            motion_blur,
            blend_mode,
            css_class,
            id,
            tag_name,
            tranform_before_mask_deprecated,
            transform_before_mask,
        })
    }
}
