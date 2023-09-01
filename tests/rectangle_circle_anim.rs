pub const JSON: &str = include_str!("rectangle_circle_anim.json");

use vellottie::parser::Lottie;

#[test]
fn test_deserialize() {
    let actual = Lottie::from_str(JSON);

    if let Err(e) = actual {
        panic!("{e}");
    }
}
