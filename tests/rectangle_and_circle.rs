pub const JSON: &str = include_str!("rectangle_and_circle.json");

use vellottie::{parser::Lottie, *};

#[test]
fn test_deserialize() {
    let actual = Lottie::from_str(JSON);

    match actual {
        Err(e) => panic!("{e}"),
        _ => {}
    }
}
