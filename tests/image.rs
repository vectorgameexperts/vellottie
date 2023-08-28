use lazy_static::lazy_static;
use serde_json::{json, Number};
use vellottie::parser::{
    breadcrumb::Breadcrumb,
    models::{assets::image::Image, BoolInt},
};

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "id": "my image",
            "h": 512,
            "w": 512,
            "e": 1,
            "u": "",
            "p": "data:image/png;base64,..."
        }
    );
    static ref IMAGE: Image = Image {
        name: None,
        id: "my image".to_string(),
        height: Number::from(512),
        width: Number::from(512),
        embedded: Some(BoolInt::True),
        dir: "".to_string(),
        file_name: "data:image/png;base64,...".to_string(),
        sequence: None
    };
}

#[test]
fn test_serde_deserialize() {
    let actual = serde_json::from_value(JSON.to_owned());

    match actual {
        Ok(actual) => assert_eq!(*IMAGE, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_deserialize() {
    let obj = JSON.as_object().unwrap();
    let actual = Image::from_obj(&mut Breadcrumb::new(), obj);

    match actual {
        Ok(actual) => assert_eq!(*IMAGE, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*IMAGE).unwrap();

    assert_eq!(*JSON, actual)
}
