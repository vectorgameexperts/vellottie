pub const JSON: &str = include_str!("rectangle_and_circle.rs");

use vellottie::*;

#[test]
fn test_deserialize() {
    let actual = crate::from_str(JSON);

    match actual {
        Err(e) => panic!("{e}"),
        _ => {}
    }
}
