pub mod animated_properties;
pub mod common;
pub mod enumerations;
pub mod precomposition;
pub mod shape;
pub mod transform;

use self::{common::LayerProperties, enumerations::LayerType, transform::Transform};
use super::BoolInt;
use crate::{
    breadcrumb::Breadcrumb,
    error::ValueType,
    util::{self},
    Error,
};
use precomposition::PrecompositionLayer;
use serde::{Deserialize, Serialize};
use shape::ShapeLayer;
use util::MapExt;

/// There are several layer types, which is specified by the 'ty' attribute. All layers share the properties in `layers::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Layer {
    /// Renders a Precomposition
    Precomposition(PrecompositionLayer),

    /// Static rectangle filling the canvas with a single color
    // todo SolidColor

    /// Renders an Image
    // todo Image

    /// No contents, only used for parenting
    // todo Null (empty)

    /// Has an array of shapes
    Shape(ShapeLayer),
    // Renders Text
    // todo text

    // unimplemented - Audio(AudioLayer),
    // unimplemented - VideoPlaceholder(VideoPlaceholderLayer)
    // unimplemented - Video(VideoLayer)
    // unimplemented - ImagePlaceholder(ImagePlaceholderLayer)
    // unimplemented - Guide(GuideLayer)
    // unimplemented - Adjustment(AdjustmentLayer)
    // unimplemented - Camera(CameraLayer)
    // unimplemented - Light(LightLayer)
    // unimplemented - Data(DataLayer)
}

impl Layer {
    pub fn from_json(breadcrumb: &mut Breadcrumb, v: &serde_json::Value) -> Result<Layer, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Layer,
        })?;
        let name = root.extract_string(breadcrumb, "nm").ok();
        breadcrumb.enter(name.clone().unwrap_or("(unnamed layer)".to_string()));

        let properties = LayerProperties::from_obj(breadcrumb, root)?;

        let layer = match properties.layer_type {
            LayerType::Precomposition => Layer::Precomposition(
                PrecompositionLayer::from_properties_and_object(breadcrumb, properties, root)?,
            ),
            LayerType::Shape => Layer::Shape(ShapeLayer::from_properties_and_object(
                breadcrumb, properties, root,
            )?),
            _ => todo!(),
        };
        breadcrumb.exit();
        Ok(layer)
    }
}