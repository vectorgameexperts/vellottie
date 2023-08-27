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
            "ind": 0,
            "st": 0,
            "ip": 0,
            "op": 180,
            "nm": "Rectangle",
            "mn": "{50b511ea-ca42-416e-b630-58eca8fb41d9}",
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
                    "nm": "Rectangle",
                    "mn": "{c338119b-6435-437a-a855-0f36a8264c22}",
                    "it": [
                        {
                            "ty": "rc",
                            "nm": "Rectangle",
                            "mn": "{9e7f37be-ac4b-411e-8e47-3e2a2cad9102}",
                            "p": {
                                "a": 0,
                                "k": [
                                    185.69552238805971,
                                    256.1910447761194
                                ]
                            },
                            "s": {
                                "a": 0,
                                "k": [
                                    268.2268656716418,
                                    225.24179104477614
                                ]
                            },
                            "r": {
                                "a": 0,
                                "k": 0
                            }
                        },
                        {
                            "ty": "st",
                            "hd": true,
                            "nm": "Stroke",
                            "mn": "{9781440d-fa0d-45e1-b224-8c96fefc1f09}",
                            "o": {
                                "a": 0,
                                "k": 100
                            },
                            "c": {
                                "a": 0,
                                "k": [
                                    0,
                                    0.5019607843137255,
                                    1
                                ]
                            },
                            "lc": 2,
                            "lj": 2,
                            "ml": 0,
                            "w": {
                                "a": 0,
                                "k": 1
                            }
                        },
                        {
                            "ty": "fl",
                            "nm": "Fill",
                            "mn": "{dc302cc0-3912-40b0-81da-03b5c172432c}",
                            "o": {
                                "a": 0,
                                "k": 100
                            },
                            "c": {
                                "a": 0,
                                "k": [
                                    0.19607843137254902,
                                    0.3137254901960784,
                                    0.6901960784313725
                                ]
                            },
                            "r": 1
                        },
                        {
                            "ty": "tr",
                            "a": {
                                "a": 0,
                                "k": [
                                    185.69552238805971,
                                    256.1910447761194
                                ]
                            },
                            "p": {
                                "a": 0,
                                "k": [
                                    212.34626865671646,
                                    239.85671641791043
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
            three_dimensional: BoolInt::False,
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

#[test]
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
