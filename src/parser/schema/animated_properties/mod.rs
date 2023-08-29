#[allow(clippy::module_inception)]
mod animated_properties;
pub use animated_properties::*;

pub mod animated_property;
//todo pub mod color_value;
//todo pub mod shape_keyframe;
pub mod keyframe_base;
pub mod multi_dimensional;
//todo pub mod shape_property;
pub mod keyframe;
pub mod keyframe_bezier_handle;
pub mod position;
//todo pub mod gradient_colors;
//todo pub mod split_vector;
pub mod position_keyframe;
pub mod value;
