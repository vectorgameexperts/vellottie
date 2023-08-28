pub mod assets;
pub mod layer;
pub mod lottie;
pub mod shapes;

use serde_repr::{Deserialize_repr, Serialize_repr};
#[derive(
    Deserialize_repr, Serialize_repr, PartialEq, Default, Debug, Clone,
)]
#[repr(u8)]
pub enum BoolInt {
    #[default]
    False = 0,
    True = 1,
}
