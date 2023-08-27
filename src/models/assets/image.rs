use crate::breadcrumb::Breadcrumb;
use crate::models::BoolInt;
use crate::util::{self, MapExt};
use crate::Error;
use serde::{de::Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Represents a (static) image
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Image {
    /// Unique identifier used by layers when referencing this asset
    #[serde(rename = "id")]
    pub id: String,
    /// Human readable name
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path to the directory containing a file
    #[serde(rename = "u")]
    pub dir: String,
    /// Filename or data url
    #[serde(rename = "p")]
    pub file_name: String,
    /// Whether the file is embedded
    #[serde(rename = "e", default)]
    #[serde(skip_serializing_if = "util::is_false_int")]
    pub embedded: BoolInt,
    /// Width of the image
    #[serde(rename = "w")]
    pub width: Number,
    /// Height of the image
    #[serde(rename = "h")]
    pub height: Number,
    /// Mark as part of an image sequence if present.
    #[serde(
        rename = "t",
        deserialize_with = "seq_from_str",
        serialize_with = "seq_to_str",
        default
    )]
    #[serde(skip_serializing_if = "util::is_false")]
    pub sequence: bool,
}

pub fn seq_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    if String::deserialize(deserializer)? == *"seq" {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn seq_to_str<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        true => Serializer::serialize_str(serializer, "seq"),
        false => unimplemented!("serializer should skip if false"),
    }
}

impl Image {
    pub fn from_object(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let id = obj.extract_string(breadcrumb, "id")?;
        let name = obj.extract_string(breadcrumb, "nm").ok();
        let dir = obj.extract_string(breadcrumb, "u")?;
        let file_name = obj.extract_string(breadcrumb, "p")?;
        let embedded = obj.extract_bool_int(breadcrumb, "e")?;
        let width = obj.extract_number(breadcrumb, "w")?;
        let height = obj.extract_number(breadcrumb, "h")?;
        let sequence = obj
            .extract_string(breadcrumb, "t")
            .is_ok_and(|t| t == *"seq");
        Ok(Self {
            id,
            name,
            dir,
            file_name,
            embedded,
            width,
            height,
            sequence,
        })
    }
}