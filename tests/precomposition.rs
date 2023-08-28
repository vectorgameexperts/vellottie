#![recursion_limit = "512"]

use lazy_static::lazy_static;
use serde_json::{json, Number};
use vellottie::parser::{
    breadcrumb::Breadcrumb,
    schema::{
        assets::precomposition::Precomposition, helpers::int_boolean::BoolInt,
    },
};

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "id": "precomp_0",
            "fr": 60,
            "nm": "Example",
            "xt": 0,
            "layers": []
        }
    );
    static ref PRECOMP: Precomposition = Precomposition {
        id: "precomp_0".to_string(),
        name: Some("Example".to_string()),
        frame_rate: Some(Number::from(60)),
        extra: Some(BoolInt::False),
        layers: vec![]
    };
}

#[test]
fn test_serde_deserialize() {
    let actual = serde_json::from_value(JSON.to_owned());

    match actual {
        Ok(actual) => assert_eq!(*PRECOMP, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_deserialize() {
    let obj = JSON.as_object().unwrap();
    let actual = Precomposition::from_obj(&mut Breadcrumb::new(), obj);

    match actual {
        Ok(actual) => assert_eq!(*PRECOMP, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*PRECOMP).unwrap();

    assert_eq!(*JSON, actual)
}
