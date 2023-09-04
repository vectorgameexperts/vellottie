pub mod common;
pub mod enumerations;
pub mod precomposition;
pub mod shape;

use self::{common::LayerProperties, enumerations::LayerType};
use super::{animated_properties::value::FloatValue, shapes::AnyShape};
use crate::parser::{
    breadcrumb::Breadcrumb,
    breadcrumb::ValueType,
    util::{self},
    Error,
};
use precomposition::PrecompositionLayer;
use serde::{Deserialize, Serialize};
use shape::ShapeLayer;
use util::MapExt;

/// There are several layer types, which is specified by the 'ty' attribute. All
/// layers share the properties in `layers::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnyLayer {
    /// Renders a Precomposition
    Precomposition(PrecompositionLayer),

    /// Static rectangle filling the canvas with a single color
    // todo SolidColor

    /// Renders an Image
    // todo Image

    /// No contents, only used for parenting

    /// Has an array of shapes
    Shape(ShapeLayer),
    // Renders Text
    // todo Text
    Null(LayerProperties),
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

impl AnyLayer {
    pub fn from_json(
        breadcrumb: &mut Breadcrumb,
        v: &serde_json::Value,
    ) -> Result<AnyLayer, Error> {
        let root = v.as_object().ok_or(Error::UnexpectedChild {
            breadcrumb: breadcrumb.to_owned(),
            expected: ValueType::Layer,
        })?;
        let name = root.extract_string(breadcrumb, "nm").ok();
        breadcrumb.enter(ValueType::Layer, name);

        //Extract
        let properties = LayerProperties::from_obj(breadcrumb, root)?;
        let layer = match properties.layer_type {
            LayerType::Precomposition => {
                AnyLayer::Precomposition(PrecompositionLayer {
                    properties,
                    precomp_id: root.extract_string(breadcrumb, "refId")?,
                    width: root.extract_number(breadcrumb, "w")?,
                    height: root.extract_number(breadcrumb, "h")?,
                    time_remap: root
                        .extract_obj(breadcrumb, "tm")
                        .and_then(|obj| FloatValue::from_obj(breadcrumb, &obj))
                        .ok(),
                })
            }
            LayerType::Shape => AnyLayer::Shape(ShapeLayer {
                properties,
                shapes: {
                    let mut shapes = vec![];
                    let json_shapes = root.extract_arr(breadcrumb, "shapes")?;
                    breadcrumb.enter(ValueType::Array, Some("shapes"));
                    for v in json_shapes {
                        let shape = AnyShape::from_json(breadcrumb, &v)?;
                        shapes.push(shape);
                    }
                    breadcrumb.exit();
                    shapes
                },
            }),
            LayerType::Null => AnyLayer::Null(properties),
            layer_type => {
                todo!("layer type {:?} not implemented yet", layer_type)
            }
        };

        breadcrumb.exit();
        Ok(layer)
    }
}
