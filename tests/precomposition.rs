#![recursion_limit = "512"]

use lazy_static::lazy_static;
use serde_json::json;
use vellottie::{
    breadcrumb::Breadcrumb,
    models::{assets::precomposition::Precomposition, BoolInt},
};

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "id": "precomp_0",
            "layers": []
        }
    );
    static ref PRECOMP: Precomposition = Precomposition {
        id: "precomp_0".to_string(),
        name: None,
        frame_rate: None,
        extra: BoolInt::False,
        layers: vec![]
    };
}

#[test]
fn test_deserialize() {
    let obj = JSON.as_object().unwrap();
    let actual = Precomposition::from_object(&mut Breadcrumb::new(), obj);

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
