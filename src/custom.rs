use std::collections::HashMap;

use serde::de::value::MapDeserializer;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use crate::api::ApiClient;
use crate::error::{detect_common_helper_error, Error, Result};
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
        T::deserialize(MapDeserializer::new(data.into_iter()))
            .map_err(|_| Error::DecodeJsonToStruct)
    }
}

/// Builder to create new Custom Struct
pub struct CustomBuilder<'a> {
    api_client: &'a ApiClient,
    function: &'a str,
    extras: Vec<(&'a str, &'a str)>,
}

impl<'a> CustomBuilder<'a> {
    /// Create new `CustomBuilder` from `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, function: &'a str) -> Self {
        Self {
            api_client,
            function,
            extras: vec![],
        }
    }

    /// Add extra parameter to url
    pub fn extra_params(&mut self, key: &'a str, value: &'a str) -> &mut Self {
        self.extras.push((key, value));
        self
    }

    fn create_url(&self) -> String {
        let mut path = format!("query?function={}", self.function);
        for (key, value) in &self.extras {
            path.push_str(format!("&{key}={value}").as_str());
        }

        path
    }

    /// Returns JSON data struct
    ///
    /// # Errors
    /// Raise error if data obtained cannot be properly converted to struct or
    /// API returns any 4 possible known errors
    pub async fn json<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.create_url();
        let custom_helper: CustomHelper = self.api_client.get_json(&url).await?;
        custom_helper.convert()
    }
}
