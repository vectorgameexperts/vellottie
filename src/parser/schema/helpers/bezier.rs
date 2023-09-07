use super::int_boolean::BoolInt;
use crate::parser::{
    breadcrumb::{Breadcrumb, ValueType},
    util::{MapExt, ValueExt},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// This represents a cubic bezier path.
/// Note that for interpolation to work correctly all bezier values in a property's keyframe must have the same number of points.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Bezier {
    /// Whether the bezier forms a closed loop
    #[serde(rename = "c")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,

    /// Points along the curve
    #[serde(rename = "v")]
    pub vertices: Vec<[Number; 2]>,

    /// Cubic control points, incoming tangent
    #[serde(rename = "i")]
    pub in_tangents: Vec<[Number; 2]>,

    /// Cubic control points, outgoing tangent
    #[serde(rename = "o")]
    pub out_tangents: Vec<[Number; 2]>,
}

impl Bezier {
    pub fn from_obj(
        breadcrumb: &mut Breadcrumb,
        obj: &serde_json::map::Map<String, Value>,
    ) -> Result<Self, Error> {
        breadcrumb.enter_unnamed(ValueType::Bezier);
        let closed = obj.extract_bool(breadcrumb, "c").ok();

        let vertices = obj.extract_arr(breadcrumb, "v")?;
        breadcrumb.enter(ValueType::Array, Some("v"));
        let vertices = {
            let mut mapped_verts = vec![];
            for v in vertices {
                mapped_verts.push(v.cast_dyn::<[Number; 2]>(
                    breadcrumb,
                    ValueType::Scalar2d,
                )?);
            }
            mapped_verts
        };
        breadcrumb.exit();

        let in_tangents = obj.extract_arr(breadcrumb, "i")?;
        breadcrumb.enter(ValueType::Array, Some("l"));
        let in_tangents = {
            let mut mapped_verts = vec![];
            for v in in_tangents {
                mapped_verts.push(v.cast_dyn::<[Number; 2]>(
                    breadcrumb,
                    ValueType::Scalar2d,
                )?);
            }
            mapped_verts
        };
        breadcrumb.exit();
        let out_tangents = obj.extract_arr(breadcrumb, "o")?;
        breadcrumb.enter(ValueType::Array, Some("o"));
        let out_tangents = {
            let mut mapped_verts = vec![];
            for v in out_tangents {
                mapped_verts.push(v.cast_dyn::<[Number; 2]>(
                    breadcrumb,
                    ValueType::Scalar2d,
                )?);
            }
            mapped_verts
        };
        breadcrumb.exit();

        let bezier = Self {
            closed,
            vertices,
            in_tangents,
            out_tangents,
        };
        breadcrumb.exit();
        Ok(bezier)
    }
}
