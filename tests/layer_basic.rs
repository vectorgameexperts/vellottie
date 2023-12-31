#![recursion_limit = "512"]
#![allow(deprecated)]
use lazy_static::lazy_static;
use serde_json::{json, Number};
use vellottie::parser::schema::animated_properties::animated_property::AnimatedPropertyK;
use vellottie::parser::schema::animated_properties::position::{
    Position, PositionValueK,
};
use vellottie::parser::schema::helpers::transform::{
    AnyTransformP, AnyTransformR, Transform,
};
use vellottie::parser::schema::helpers::visual_object::VisualObject;
use vellottie::parser::schema::shapes::shape_element::ShapeElement;
use vellottie::parser::{
    breadcrumb::Breadcrumb,
    schema::{
        animated_properties::{
            animated_property::AnimatedProperty,
            multi_dimensional::MultiDimensional, value::FloatValue,
        },
        helpers::int_boolean::BoolInt,
        layers::{shape::ShapeLayer, visual::VisualLayer, AnyLayer},
        shapes::{ellipse::EllipseShape, group::GroupShape, AnyShape},
    },
};

lazy_static! {
    static ref JSON: serde_json::Value = json!(
        {
            "ddd": 0,
            "ty": 4,
            "ind": 1,
            "st": 0,
            "ip": 0,
            "op": 180,
            "nm": "Ellipse",
            "mn": "{0a36d01c-18e1-48d3-8e8f-cc093b3f24ba}",
            "ks": {
                "a": {
                    "a": 0,
                    "k": [
                        256,
                        256
                    ]
                },
                "p": {
                    "a": 0,
                    "k": [
                        256,
                        256
                    ]
                },
                "s": {
                    "a": 0,
                    "k": [
                        100,
                        100
                    ]
                },
                "r": {
                    "a": 0,
                    "k": 0
                },
                "o": {
                    "a": 0,
                    "k": 100
                }
            },
            "shapes": [
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
            ]
        }
    );
    static ref LAYER: AnyLayer = AnyLayer::Shape(ShapeLayer {
        properties: VisualLayer {
            name: Some("Ellipse".to_string()),
            match_name: Some(
                "{0a36d01c-18e1-48d3-8e8f-cc093b3f24ba}".to_string()
            ),
            three_dimensional: Some(BoolInt::False),
            index: Some(Number::from(1)),
            start_time: Number::from(0),
            in_point: Number::from(0),
            out_point: Number::from(180),
            transform: Transform {
                anchor_point: Some(Position {
                    property_index: None,
                    animated: Some(BoolInt::False),
                    expression: None,
                    length: None,
                    value: PositionValueK::Static(vec![
                        Number::from(256),
                        Number::from(256)
                    ],),
                }),
                position: AnyTransformP::Position(Position {
                    property_index: None,
                    animated: Some(BoolInt::False),
                    expression: None,
                    length: None,
                    value: PositionValueK::Static(vec![
                        Number::from(256),
                        Number::from(256)
                    ],),
                }),
                scale: Some(MultiDimensional {
                    animated_property: AnimatedProperty {
                        animated: Some(BoolInt::False),
                        property_index: None,
                        expression: None,
                        slot_id: None,
                        value: AnimatedPropertyK::Static(vec![
                            Number::from(100),
                            Number::from(100)
                        ])
                    },
                    length: None,
                }),
                rotation: Some(AnyTransformR::Rotation(FloatValue {
                    animated_property: AnimatedProperty {
                        animated: Some(BoolInt::False),
                        property_index: None,
                        expression: None,
                        slot_id: None,
                        value: AnimatedPropertyK::Static(Number::from(0))
                    },
                })),
                opacity: Some(FloatValue {
                    animated_property: AnimatedProperty {
                        animated: Some(BoolInt::False),
                        property_index: None,
                        expression: None,
                        slot_id: None,
                        value: AnimatedPropertyK::Static(Number::from(100))
                    }
                }),
                skew: None,
                skew_axis: None,
            },
            hidden: None,
            parent_index: None,
            time_stretch: None,
            matte_mode: None,
            matte_target: None,
            masks_properties: None,
            rotate_to_match_anim_pos_path: None,
            matte_layer_index: None,
            has_mask: None,
            motion_blur: None,
            blend_mode: None,
            css_class: None,
            id: None,
            tag_name: None,
            tranform_before_mask_deprecated: None,
            transform_before_mask: None
        },
        layer_type: vellottie::parser::schema::layers::shape::LayerId::Shape,
        shapes: vec![AnyShape::Group(GroupShape {
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
                            "{2aabac6e-1dd8-41b0-b60b-baf75ccb6318}"
                                .to_string()
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
        }),]
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
    let actual = AnyLayer::from_json(&mut Breadcrumb::new(), &JSON);

    match actual {
        Ok(actual) => assert_eq!(*LAYER, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_xor_deserialize() {
    // Ensure our (slow) parsing method returns the same result as serde's.
    let vellottie_parse =
        AnyLayer::from_json(&mut Breadcrumb::new(), &JSON).unwrap();
    let serde_parse = serde_json::from_value(JSON.to_owned()).unwrap();

    assert_eq!(vellottie_parse, serde_parse);
}

#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*LAYER).unwrap();

    assert_eq!(*JSON, actual)
}
