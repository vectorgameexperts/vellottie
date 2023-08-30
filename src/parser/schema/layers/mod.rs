pub mod common;
pub mod enumerations;
pub mod precomposition;
pub mod shape;
pub mod text;

use self::{common::LayerProperties, enumerations::LayerType};
use super::{animated_properties::AnimatedNumber, shapes::Shape};
use crate::parser::{
    breadcrumb::Breadcrumb,
    breadcrumb::ValueType,
    util::{self},
    Error,
};
use precomposition::PrecompositionLayer;
use serde::{Deserialize, Serialize};
use shape::ShapeLayer;
use text::TextLayer;
use util::MapExt;

/// There are several layer types, which is specified by the 'ty' attribute. All
/// layers share the properties in `layers::common::Properties`.
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
    Text(TextLayer),
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
    pub fn from_json(
        breadcrumb: &mut Breadcrumb,
        v: &serde_json::Value,
    ) -> Result<Layer, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Layer,
        })?;
        let name = root.extract_string(breadcrumb, "nm").ok();
        breadcrumb.enter(ValueType::Layer, name.clone());

        //Extract
        let properties = LayerProperties::from_obj(breadcrumb, root)?;
        let layer = match properties.layer_type {
            LayerType::Precomposition => {
                Layer::Precomposition(PrecompositionLayer {
                    properties,
                    precomp_id: root.extract_string(breadcrumb, "refID")?,
                    width: root.extract_number(breadcrumb, "w")?,
                    height: root.extract_number(breadcrumb, "h")?,
                    time_remap: AnimatedNumber::from_obj(breadcrumb, root)?,
                })
            }
            LayerType::Shape => Layer::Shape(ShapeLayer {
                properties,
                shapes: {
                    let mut shapes = vec![];
                    let json_shapes = root.extract_arr(breadcrumb, "shapes")?;
                    breadcrumb.enter(ValueType::Array, Some("shapes"));
                    for v in json_shapes {
                        let shape = Shape::from_json(breadcrumb, &v)?;
                        shapes.push(shape);
                    }
                    breadcrumb.exit();
                    shapes
                },
            }),
            _ => todo!(),
        };

        breadcrumb.exit();
        Ok(layer)
    }
}
