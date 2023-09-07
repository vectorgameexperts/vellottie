use crate::parser::schema::animated_properties::multi_dimensional::MultiDimensional;
use crate::parser::schema::animated_properties::value::FloatValue;
use crate::parser::schema::helpers::int_boolean::BoolInt;

use crate::parser::schema;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FLOAT_VALUE_ZERO: FloatValue = FloatValue {
        animated_property:
            schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: Some(BoolInt::False),
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    serde_json::Number::from(0),
                ),
            },
    };
    pub static ref FLOAT_VALUE_ONE_HUNDRED: FloatValue = FloatValue {
        animated_property:
            schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: Some(BoolInt::False),
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    serde_json::Number::from(100),
                ),
            },
    };

    pub static ref MULTIDIM_ZERO: MultiDimensional =
        MultiDimensional {
            animated_property: schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: Some(BoolInt::False),
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    vec![serde_json::Number::from(0), serde_json::Number::from(0), serde_json::Number::from(0)],
                ),
            },
        };
    pub static ref MULTIDIM_ONE: MultiDimensional =
        MultiDimensional {
            animated_property: schema::animated_properties::animated_property::AnimatedProperty {
                property_index: None,
                animated: Some(BoolInt::False),
                expression: None,
                slot_id: None,
                value: schema::animated_properties::animated_property::AnimatedPropertyK::Static(
                    vec![serde_json::Number::from(1), serde_json::Number::from(1), serde_json::Number::from(1)],
                ),
            },
        };

    pub static ref POSITION_ZERO: schema::animated_properties::position::Position = schema::animated_properties::position::Position {
        property_index: None,
        animated: Some(BoolInt::False),
        expression: None,
        length: None,
        value: schema::animated_properties::position::PositionValueK::Static(vec![serde_json::Number::from(0), serde_json::Number::from(0)]),
    };

}
