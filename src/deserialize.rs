use serde::de::{Deserialize, Deserializer, Error};
use std::str::FromStr;

pub(crate) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(Error::custom)
}

pub(crate) fn from_none_str<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    if &s == "none" {
        Ok(None)
    } else {
        match T::from_str(&s) {
            Ok(data) => Ok(Some(data)),
            Err(msg) => Err(msg),
        }
        .map_err(Error::custom)
    }
}

pub(crate) fn percent_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;
    s.pop();
    f64::from_str(&s).map_err(Error::custom)
}
