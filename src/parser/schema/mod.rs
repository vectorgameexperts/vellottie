//! The schema for a Lottie `Animation` type.
//!
//! Schema: https://lottiefiles.github.io/lottie-docs/schema/lottie.schema.json

pub mod animated_properties;
pub mod animation;
pub mod assets;
// todo constants
// todo effect-values
// todo effects
pub mod helpers;
pub mod layers;
pub mod lottie; // this should be "animation"
pub mod shapes;
pub mod styles;
//todo text
pub mod transform; // this should be under "helpers"
