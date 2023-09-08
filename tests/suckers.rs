pub const JSON: &str = include_str!("suckers.json");

use serde_json::Value;
use vellottie::parser::{breadcrumb::Breadcrumb, schema::layers::AnyLayer};

#[test]
fn test_serde_deserialize() {
    let actual = serde_json::from_str::<AnyLayer>(JSON);

    match actual {
        Ok(actual) => {
            // todo assert_eq!(*GOLDEN_MODEL, actual)
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_deserialize() {
    let json = serde_json::from_str(JSON).unwrap();
    let actual = AnyLayer::from_json(&mut Breadcrumb::new(), &json);

    match actual {
        Ok(actual) => {
            // todo assert_eq!(*GOLDEN_MODEL, actual)
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_xor_deserialize() {
    // Ensure our (slow) parsing method returns the same result as serde's.
    let json = serde_json::from_str(JSON).unwrap();
    let vellottie_parse =
        AnyLayer::from_json(&mut Breadcrumb::new(), &json).unwrap();
    let serde_parse = serde_json::from_str(JSON).unwrap();

    assert_eq!(vellottie_parse, serde_parse);
}

#[test]
fn test_serialize() {
    let json: Value = serde_json::from_str(JSON).unwrap();

    // todo use golden model
    let shape: AnyLayer =
        AnyLayer::from_json(&mut Breadcrumb::new(), &json).unwrap();
    let actual = serde_json::to_value(shape).unwrap();

    assert_eq!(json, actual)
}
