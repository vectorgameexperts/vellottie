#![recursion_limit = "512"]
#![allow(deprecated)]
use lazy_static::lazy_static;
use serde_json::{json, Number};
use vellottie::parser::schema::animated_properties::animated_property::AnimatedPropertyK;
use vellottie::parser::schema::animated_properties::position::{
    Position, PositionValueK,
};
use vellottie::parser::schema::helpers::visual_object::VisualObject;
use vellottie::parser::schema::shapes::shape_element::ShapeElement;
use vellottie::parser::{
    breadcrumb::Breadcrumb,
    schema::{
        animated_properties::{
            animated_property::AnimatedProperty,
            multi_dimensional::MultiDimensional,
        },
        helpers::int_boolean::BoolInt,
        shapes::{ellipse::EllipseShape, group::GroupShape, AnyShape},
    },
};

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "ty": "gr",
            "nm": "Group",
            "mn": "{f1becc2a-49f0-4f0c-918f-bdffe4c6870f}",
            "it": [
                {
                    "ty": "el",
                    "nm": "Ellipse",
                    "mn": "{2aabac6e-1dd8-41b0-b60b-baf75ccb6318}",
                    "p": {
                        "a": 0,
                        "k": [
                            303.9044776119403,
                            324.9671641791045
                        ]
                    },
                    "s": {
                        "a": 0,
                        "k": [
                            205.46865671641788,
                            204.6089552238806
                        ]
                    }
                }
            ]
        }
    );
    static ref LAYER: AnyShape = AnyShape::Group(GroupShape {
        shape_element: ShapeElement {
            visual_object: VisualObject {
                name: Some("Group".to_string()),
                match_name: Some(
                    "{f1becc2a-49f0-4f0c-918f-bdffe4c6870f}".to_string()
                )
            },
            index: None,
            hidden: None,
            blend_mode: None,
            property_index: None,
            css_class: None,
            xml_id: None
        },
        num_properties: None,
        property_index: None,
        shapes: vec![AnyShape::Ellipse(EllipseShape {
            shape_element: ShapeElement {
                visual_object: VisualObject {
                    name: Some("Ellipse".to_string()),
                    match_name: Some(
                        "{2aabac6e-1dd8-41b0-b60b-baf75ccb6318}".to_string()
                    )
                },
                index: None,
                hidden: None,
                blend_mode: None,
                property_index: None,
                css_class: None,
                xml_id: None
            },
            position: Position {
                property_index: None,
                animated: Some(BoolInt::False),
                expression: None,
                length: None,
                value: PositionValueK::Static(vec![
                    Number::from_f64(303.9044776119403).unwrap(),
                    Number::from_f64(324.9671641791045).unwrap()
                ],),
            },
            size: MultiDimensional {
                animated_property: AnimatedProperty {
                    animated: Some(BoolInt::False),
                    property_index: None,
                    expression: None,
                    slot_id: None,
                    value: AnimatedPropertyK::Static(vec![
                        Number::from_f64(205.46865671641788).unwrap(),
                        Number::from_f64(204.6089552238806).unwrap()
                    ])
                },
                length: None,
            },
        })]
    });
}

#[test]
fn test_serde_deserialize() {
    let actual = serde_json::from_value(JSON.to_owned());

    match actual {
        Ok(actual) => assert_eq!(*LAYER, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_deserialize() {
    let actual = AnyShape::from_json(&mut Breadcrumb::new(), &JSON);

    match actual {
        Ok(actual) => assert_eq!(*LAYER, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_xor_deserialize() {
    // Ensure our (slow) parsing method returns the same result as serde's.
    let vellottie_parse =
        AnyShape::from_json(&mut Breadcrumb::new(), &JSON).unwrap();
    let serde_parse = serde_json::from_value(JSON.to_owned()).unwrap();

    assert_eq!(vellottie_parse, serde_parse);
}

#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*LAYER).unwrap();

    assert_eq!(*JSON, actual)
}
