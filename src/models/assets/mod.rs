pub mod image;
pub mod precomposition;

use self::{image::Image, precomposition::Precomposition};
use crate::{breadcrumb::Breadcrumb, error::ValueType, util::MapExt, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Asset {
    Image(Image),
    Precomposition(Precomposition),
    // unimplemented - Sound(Sound),
    // unimplemented - DataSource(DataSource),
}

impl Asset {
    pub fn from_json(breadcrumb: &mut Breadcrumb, v: &serde_json::Value) -> Result<Self, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Object,
        })?;
        let id = root.extract_string(breadcrumb, "id")?;
        breadcrumb.enter(id);
        // Extract
        let asset = if root.contains_key("layers") {
            // Asset is a precomposition
            Asset::Precomposition(Precomposition::from_object(breadcrumb, root)?)
        } else if root.contains_key("p") {
            // Asset is an image
            Asset::Image(Image::from_object(breadcrumb, root)?)
        } else {
            return Err(Error::UnexpectedChild {
                breadcrumb: breadcrumb.clone(),
                expected: ValueType::Asset,
            });
        };
        breadcrumb.exit();
        Ok(asset)
    }
}
