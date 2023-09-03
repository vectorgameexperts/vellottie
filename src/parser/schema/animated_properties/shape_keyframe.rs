use super::keyframe_base::KeyframeBase;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    schema::helpers::bezier::Bezier,
    util::MapExt,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Keyframe holding Bezier objects
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeKeyframe {
    #[serde(flatten)]
    pub base: KeyframeBase,
    #[serde(rename = "s")]
    pub start: Vec<Bezier>,
}

impl ShapeKeyframe {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        let base = KeyframeBase::from_obj(breadcrumb, obj)?;
        let beziers = obj.extract_arr(breadcrumb, "s")?;
        breadcrumb.enter(ValueType::Array, Some("s"));
        let start = {
            let mut mapped_beziers = vec![];
            for bezier in beziers {
                let bezier_obj =
                    bezier.as_object().ok_or(Error::UnexpectedChild {
                        breadcrumb: breadcrumb.clone(),
                        expected: ValueType::Object,
                    })?;
                mapped_beziers.push(Bezier::from_obj(breadcrumb, bezier_obj)?);
            }
            mapped_beziers
        };
        breadcrumb.exit();
        Ok(Self { base, start })
    }
}
