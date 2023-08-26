use crate::{error::ValueType, Error};
use serde_json::{Map, Value};
use std::{borrow::Borrow, fmt::Display, hash::Hash};

pub(crate) trait MapExt {
    fn extract_string<Q>(&self, parent: &Q, key: &Q) -> Result<String, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;
}

//impl ValueUtil for serde_json::Value {
//    fn extract_string<I>(parent: &str, v: &serde_json::Value, key: I) -> Result<String, Error>
//    where
//        I: Index + Display,
//    {
//        Ok(v.get(&key)
//            .ok_or_else(|| Error::MissingChild(parent.to_string(), key.to_string()))?
//            .as_str()
//            .ok_or_else(|| Error::IncorrectType(key.to_string(), ValueType::String))?
//            .to_owned())
//    }
//}

impl MapExt for &Map<String, Value> {
    fn extract_string<Q>(&self, parent: &Q, key: &Q) -> Result<String, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .get(key)
            .ok_or_else(|| Error::MissingChild(parent.to_string(), key.to_string()))?
            .as_str()
            .ok_or_else(|| Error::IncorrectType(key.to_string(), ValueType::String))?
            .to_owned())
    }
}
