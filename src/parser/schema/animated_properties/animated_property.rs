use super::keyframe::Keyframe;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::int_boolean::BoolInt,
    util::MapExt,
    Error,
};
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};
use serde_json::{Number, Value};

/// An animatable property that holds an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AnimatedProperty<StaticType> {
    /// Property Index
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<Number>,
    /// Whether the property is animated.
    #[serde(rename = "a")]
    pub animated: BoolInt,
    /// Expression for the property.
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// One of the ID in the file's slots
    #[serde(rename = "sid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
    #[serde(rename = "k")]
    #[serde(bound = "StaticType: Serialize + DeserializeOwned")]
    pub value: AnimatedPropertyK<StaticType>,
}

/// The possible values of "k" in an [`AnimatedProperty`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnimatedPropertyK<StaticType> {
    /// Array of keyframes
    AnimatedValue(Vec<Keyframe>),
    /// Static value
    Static(StaticType),
}

impl<StaticType> AnimatedProperty<StaticType>
where
    StaticType: de::DeserializeOwned + Serialize,
{
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let animated = obj.extract_bool_int(breadcrumb, "a")?;
        let prop = if animated == BoolInt::True {
            todo!();
        } else {
            AnimatedProperty {
                property_index: obj.extract_number(breadcrumb, "ix").ok(),
                animated,
                expression: obj.extract_string(breadcrumb, "x").ok(),
                slot_id: obj.extract_string(breadcrumb, "sid").ok(),
                value: AnimatedPropertyK::Static(obj.extract_type(
                    breadcrumb,
                    "k",
                    ValueType::Value,
                )?),
            }
        };
        Ok(prop)
    }
}
