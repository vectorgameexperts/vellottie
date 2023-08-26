use lazy_static::lazy_static;
use serde_json::json;
use vellottie::*;

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "v": "5.5.2",
            "fr": 60,
            "ip": 0,
            "op": 60,
            "w": 512,
            "h": 512,
            "ddd": 0,
            "assets": [],
            "fonts": {
                "list": []
            },
            "layers": []
        }
    );
    static ref LOTTIE: Lottie = Lottie {
        version: "5.5.2".to_string(),
        frame_rate: 60.0,
        in_point: 0.0,
        out_point: 60.0,
        width: 512,
        height: 512,
        three_dimensional: false,
    };
}

#[test]
fn test_deserialize() {
    let actual = crate::from_json(&JSON);

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
