#![recursion_limit = "512"]

use lazy_static::lazy_static;
use serde_json::{json, Number};
use vellottie::{
    breadcrumb::Breadcrumb,
    models::{
        layer::{
            animated_properties::{AnimatedNumber, AnimatedVector, StaticNumber, StaticVector},
            common::LayerProperties,
            enumerations::LayerType,
            shape::ShapeLayer,
            transform::Transform,
            Layer,
        },
        BoolInt,
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
                    "nm": "Ellipse",
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
                        },
                        {
                            "ty": "tr",
                            "a": {
                                "a": 0,
                                "k": [
                                    303.9044776119403,
                                    324.9671641791045
                                ]
                            },
                            "p": {
                                "a": 0,
                                "k": [
                                    330.55522388059705,
                                    308.63283582089554
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
                        }
                    ]
                }
            ]
        }
    );
    static ref LAYER: Layer = Layer::Shape(ShapeLayer {
        properties: LayerProperties {
            name: Some("Rectangle".to_string()),
            match_name: Some("{50b511ea-ca42-416e-b630-58eca8fb41d9}".to_string()),
            three_dimensional: None,
            layer_type: LayerType::Shape,
            index: Some(Number::from(0)),
            start_time: Number::from(0),
            in_point: Number::from(0),
            out_point: Number::from(180),
            transform: Transform {
                anchor_point: Some(AnimatedVector::Static(StaticVector {
                    animated: BoolInt::False,
                    value: [Number::from(256), Number::from(256)],
                })),
                position: Some(AnimatedVector::Static(StaticVector {
                    animated: BoolInt::False,
                    value: [Number::from(256), Number::from(256)],
                })),
                scale: Some(AnimatedVector::Static(StaticVector {
                    animated: BoolInt::False,
                    value: [Number::from(100), Number::from(100)],
                })),
                rotation: Some(AnimatedNumber::Static(StaticNumber {
                    animated: BoolInt::False,
                    value: Number::from(0),
                })),
                opacity: Some(AnimatedNumber::Static(StaticNumber {
                    animated: BoolInt::False,
                    value: Number::from(100),
                })),
                skew: None,
                skew_axis: None,
            },
            hidden: false,
            parent_index: None,
            time_stretch: None,
            matte_mode: None,
            matte_target: BoolInt::False,
            mask_properties: None,
            effects: None,
            styles: None,
            rotate_to_match_anim_pos_path: BoolInt::False,
            matte_layer_index: None,
            has_mask: false,
            motion_blur: false,
            blend_mode: None,
            css_class: None,
            id: None,
            tag_name: None,
            tranform_before_mask_deprecated: None,
            transform_before_mask: BoolInt::False
        },
        shapes: vec![]
    });
}

#[test_log::test]
fn test_deserialize() {
    let actual = Layer::from_json(&mut Breadcrumb::new(), &JSON);

    match actual {
        Ok(actual) => assert_eq!(*LAYER, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*LAYER).unwrap();

    assert_eq!(*JSON, actual)
}
