use std::collections::HashMap;

use serde::{
    de::{value::MapDeserializer, DeserializeOwned},
    Deserialize,
};
use serde_json::Value;

use crate::{
    error::{Error, Result},
    utils::detect_common_helper_error,
};
/// struct used for helping creation of custom url
#[derive(Debug, Deserialize)]
pub(crate) struct CustomHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(flatten)]
    extras: HashMap<String, Value>,
}

impl CustomHelper {
    pub(crate) fn convert<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        detect_common_helper_error(self.information, self.error_message, self.note)?;
        let data = self.extras;
        dbg!(&data);
        T::deserialize(MapDeserializer::new(data.into_iter()))
            .map_err(|_| Error::DecodeJsonToStruct)
    }
}
