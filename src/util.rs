use crate::{error::ValueType, Error};
use serde::{
    de::{self, Deserialize as _, Deserializer, Unexpected},
    Serializer,
};
use serde_json::{Map, Value};
use std::{borrow::Borrow, fmt::Display, hash::Hash};

pub trait MapExt {
    fn extract_value<Q>(&self, parent: &Q, key: &Q) -> Result<&Value, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_string<Q>(&self, parent: &Q, key: &Q) -> Result<String, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_number<Q>(&self, parent: &Q, key: &Q) -> Result<f64, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_int<Q>(&self, parent: &Q, key: &Q) -> Result<i64, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_bool<Q>(&self, parent: &Q, key: &Q) -> Result<bool, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;
}

impl MapExt for &Map<String, Value> {
    fn extract_value<Q>(&self, parent: &Q, key: &Q) -> Result<&Value, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        self.get(key)
            .ok_or_else(|| Error::MissingChild(parent.to_string(), key.to_string()))
    }

    fn extract_string<Q>(&self, parent: &Q, key: &Q) -> Result<String, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .extract_value(parent, key)?
            .as_str()
            .ok_or_else(|| Error::IncorrectType(key.to_string(), ValueType::String))?
            .to_owned())
    }

    fn extract_number<Q>(&self, parent: &Q, key: &Q) -> Result<f64, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .extract_value(parent, key)?
            .as_f64()
            .ok_or_else(|| Error::IncorrectType(key.to_string(), ValueType::Number))?
            .to_owned())
    }

    fn extract_int<Q>(&self, parent: &Q, key: &Q) -> Result<i64, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .extract_value(parent, key)?
            .as_i64()
            .ok_or_else(|| Error::IncorrectType(key.to_string(), ValueType::Integer))?
            .to_owned())
    }

    fn extract_bool<Q>(&self, parent: &Q, key: &Q) -> Result<bool, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        self.extract_value(parent, key)?
            .as_u64()
            .ok_or(Error::IncorrectType(key.to_string(), ValueType::Bool))
            .and_then(|i| match i {
                0 => Ok(false),
                1 => Ok(true),
                other => Err(Error::IncorrectType(key.to_string(), ValueType::Integer)),
            })
    }
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn bool_to_int<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        true => Serializer::serialize_u8(serializer, 1),
        false => Serializer::serialize_u8(serializer, 0),
    }
}
