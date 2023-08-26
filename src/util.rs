use crate::Error;
use std::borrow::Borrow;
trait ValueUtil {
    fn extract_string<Q>(v: &serde_json::Value, key: Q) -> Result<String, Error>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + std::hash::Hash;
}
