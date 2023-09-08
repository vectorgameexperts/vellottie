use lazy_static::lazy_static;
use serde_json::{json, Number};
use vellottie::parser::{schema::helpers::int_boolean::BoolInt, *};

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "v": "5.5.2",
            "nm": "Example",
            "fr": 60,
            "ip": 0,
            "op": 60,
            "w": 512,
            "h": 512,
            "ddd": 0,
            "layers": []
        }
    );
    static ref LOTTIE: Lottie = Lottie {
        version: Some("5.5.2".to_string()),
        name: Some("Example".to_string()),
        frame_rate: Number::from(60),
        in_point: Number::from(0),
        out_point: Number::from(60),
        width: Number::from(512),
        height: Number::from(512),
        three_dimensional: Some(BoolInt::False),
        layers: vec![],
        assets: None,
    };
}

#[test]
fn test_serde_deserialize() {
    let actual = serde_json::from_value(JSON.to_owned());

    match actual {
        Ok(actual) => assert_eq!(*LOTTIE, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_deserialize() {
    let actual = Lottie::from_json(JSON.to_owned());

    match actual {
        Ok(actual) => assert_eq!(*LOTTIE, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_serialize() {
    let actual = LOTTIE.to_json();

    assert_eq!(*JSON, actual)
}
