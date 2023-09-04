use lazy_static::lazy_static;
use serde_json::{json, Number};
use vellottie::parser::{
    breadcrumb::Breadcrumb,
    schema::{
        assets::{asset::Asset, file_asset::FileAsset, image::Image},
        helpers::int_boolean::BoolInt,
    },
};

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "id": "my image",
            "h": 512,
            "w": 512,
            "e": 1,
            "p": "data:image/png;base64,..."
        }
    );
    static ref IMAGE: Image = Image {
        file_asset: FileAsset {
            asset: Asset {
                id: "my image".to_string(),
                name: None
            },
            dir: None,
            file_name: "data:image/png;base64,...".to_string(),
            embedded: Some(BoolInt::True),
        },
        height: Some(Number::from(512)),
        width: Some(Number::from(512)),
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
fn test_xor_deserialize() {
    // Ensure our (slow) parsing method returns the same result as serde's.
    let vellottie_parse =
        Image::from_obj(&mut Breadcrumb::new(), JSON.as_object().unwrap())
            .unwrap();
    let serde_parse = serde_json::from_value(JSON.to_owned()).unwrap();

    assert_eq!(vellottie_parse, serde_parse);
}

#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*IMAGE).unwrap();

    assert_eq!(*JSON, actual)
}
