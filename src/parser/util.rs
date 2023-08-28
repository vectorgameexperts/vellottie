use crate::parser::schema::helpers::int_boolean::BoolInt;
use crate::parser::{breadcrumb::Breadcrumb, breadcrumb::ValueType, Error};
use log::trace;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::{borrow::Borrow, fmt::Display, hash::Hash};

pub trait MapExt {
    fn extract_value<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<&Value, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_string<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<String, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_number<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<Number, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_bool_int<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<BoolInt, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_bool<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<bool, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_arr<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<Vec<Value>, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_obj<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<Map<String, Value>, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display;

    fn extract_type<Q, T>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
        expected: ValueType,
    ) -> Result<T, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
        T: for<'a> Deserialize<'a> + Serialize;
}

impl MapExt for &Map<String, Value> {
    fn extract_value<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<&Value, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        trace!("extracting {key}");
        self.get(key).ok_or_else(|| Error::MissingChild {
            key: key.to_string(),
            breadcrumb: breadcrumb.to_owned(),
        })
    }

    fn extract_string<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<String, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .extract_value(breadcrumb, key)?
            .as_str()
            .ok_or_else(|| Error::IncorrectType {
                key: key.to_string(),
                expected: ValueType::String,
            })?
            .to_owned())
    }

    fn extract_number<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<Number, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        let value = self.extract_value(breadcrumb, key)?;
        match value {
            Value::Number(n) => Ok(n.to_owned()),
            _ => Err(Error::IncorrectType {
                key: key.to_string(),
                expected: ValueType::Number,
            }),
        }
    }

    fn extract_bool_int<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<BoolInt, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        self.extract_value(breadcrumb, key)?
            .as_u64()
            .ok_or(Error::IncorrectType {
                key: key.to_string(),
                expected: ValueType::BoolInt,
            })
            .and_then(|i| match i {
                0 => Ok(BoolInt::False),
                1 => Ok(BoolInt::True),
                _ => Err(Error::IncorrectType {
                    key: key.to_string(),
                    expected: ValueType::BoolInt,
                }),
            })
    }

    fn extract_bool<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<bool, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .extract_value(breadcrumb, key)?
            .as_bool()
            .ok_or_else(|| Error::IncorrectType {
                key: key.to_string(),
                expected: ValueType::Bool,
            })?
            .to_owned())
    }

    fn extract_arr<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<Vec<Value>, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .extract_value(breadcrumb, key)?
            .as_array()
            .ok_or_else(|| Error::IncorrectType {
                key: key.to_string(),
                expected: ValueType::Array,
            })?
            .to_owned())
    }

    fn extract_obj<Q>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
    ) -> Result<Map<String, Value>, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
    {
        Ok(self
            .extract_value(breadcrumb, key)?
            .as_object()
            .ok_or_else(|| Error::IncorrectType {
                key: key.to_string(),
                expected: ValueType::Object,
            })?
            .to_owned())
    }

    fn extract_type<Q, T>(
        &self,
        breadcrumb: &Breadcrumb,
        key: &Q,
        expected: ValueType,
    ) -> Result<T, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash + Display,
        T: for<'a> Deserialize<'a> + Serialize,
    {
        let x = self.extract_value(breadcrumb, key)?.to_owned();
        let y: Result<T, Error> =
            serde_json::from_value(x).map_err(|_| Error::IncorrectType {
                key: key.to_string(),
                expected,
            });
        y
    }
}
