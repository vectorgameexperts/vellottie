use serde_json::Value;
use vellottie::parser::Lottie;

pub const JSON: &str = include_str!("clown.json");

#[test]
fn test_serde_deserialize() {
    let actual = serde_json::from_str::<Lottie>(JSON);

    match actual {
        Ok(_actual) => {
            // todo assert_eq!(*GOLDEN_MODEL, actual)
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_deserialize() {
    let actual = Lottie::from_str(JSON);

    match actual {
        Ok(_actual) => {
            // todo assert_eq!(*GOLDEN_MODEL, actual)
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_xor_deserialize() {
    // Ensure our (slow) parsing method returns the same result as serde's.
    let vellottie_parse = Lottie::from_str(JSON).unwrap();
    let serde_parse = Lottie::from_serde_str(JSON).unwrap();

    assert_eq!(vellottie_parse, serde_parse);
}

#[test]
fn test_serialize() {
    let json: Value = serde_json::from_str(JSON).unwrap();

    // todo use golden model
    let lottie = Lottie::from_str(JSON).unwrap();
    let actual = serde_json::to_value(lottie).unwrap();

    assert_eq!(json, actual)
}
