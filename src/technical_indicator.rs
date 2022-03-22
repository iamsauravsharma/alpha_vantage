//! Module for Technical Indicator
//!
//! Technical indicator values are updated realtime: the latest data point is
//! derived from the current trading day of a given equity or currency exchange
//! pair.
//!
//! You can read about [Technical Indicator][technical_indicator] API and what
//! it returns on alphavantage documentation
//!
//! [technical_indicator]: https://www.alphavantage.co/documentation/#technical-indicators

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::value::Value;

use crate::api::ApiClient;
use crate::error::{detect_common_helper_error, Error, Result};

type DataType = HashMap<String, HashMap<String, HashMap<String, String>>>;

/// Struct for storing a data values
#[derive(Default)]
pub struct DataCollector {
    time: String,
    values: HashMap<String, f64>,
}

impl DataCollector {
    /// Return time
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return values for Data
    #[must_use]
    pub fn values(&self) -> &HashMap<String, f64> {
        &self.values
    }
}

/// Struct for indicator
#[derive(Default, Debug)]
pub struct TechnicalIndicator {
    metadata: HashMap<String, Value>,
    data: DataType,
}

impl TechnicalIndicator {
    /// Return meta data in hash form with key as `String` and values as
    /// `serde_json::value::Value`
    #[must_use]
    pub fn meta_data(&self) -> &HashMap<String, Value> {
        &self.metadata
    }

    /// Return data as a vector
    #[must_use]
    pub fn data(&self) -> Vec<DataCollector> {
        let mut vector = Vec::new();
        for hash in self.data.values() {
            for time in hash.keys() {
                let mut data_collector = DataCollector {
                    time: time.to_string(),
                    ..DataCollector::default()
                };
                let hash_values = hash
                    .get(time)
                    .expect("cannot get time key value from hash map");

                for (key, value) in hash_values {
                    let value_f64 = value
                        .trim()
                        .parse::<f64>()
                        .expect("Cannot convert string to f64");
                    data_collector.values.insert(key.to_string(), value_f64);
                }
                vector.push(data_collector);
            }
        }
        vector
    }
}

/// Struct for helping `TechnicalIndicator` struct
#[derive(Deserialize)]
pub(crate) struct TechnicalIndicatorHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Meta Data")]
    metadata: Option<HashMap<String, Value>>,
    #[serde(flatten)]
    data: Option<DataType>,
}

impl TechnicalIndicatorHelper {
    pub(crate) fn convert(self) -> Result<TechnicalIndicator> {
        detect_common_helper_error(self.information, self.error_message, self.note)?;
        if self.metadata.is_none() || self.data.is_none() {
            return Err(Error::EmptyResponse);
        }
        Ok(TechnicalIndicator {
            metadata: self.metadata.unwrap(),
            data: self.data.unwrap(),
        })
    }
}

/// Builder to help create `TechnicalIndicator`
pub struct TechnicalIndicatorBuilder<'a> {
    api_client: &'a ApiClient<'a>,
    function: &'a str,
    symbol: &'a str,
    interval: TechnicalIndicatorInterval,
    time_period: Option<u64>,
    series_type: Option<&'a str>,
    extra_params: HashMap<String, String>,
}

impl<'a> TechnicalIndicatorBuilder<'a> {
    /// Create new `TechnicalIndicatorBuilder` form `APIClient`
    #[must_use]
    pub fn new(
        api_client: &'a ApiClient,
        function: &'a str,
        symbol: &'a str,
        interval: TechnicalIndicatorInterval,
    ) -> Self {
        Self {
            api_client,
            function,
            symbol,
            interval,
            time_period: None,
            series_type: None,
            extra_params: HashMap::new(),
        }
    }

    /// Set time period for API
    pub fn time_period(&mut self, time: u64) -> &mut Self {
        self.time_period = Some(time);
        self
    }

    /// Set series type for API
    pub fn series_type(&mut self, series_type: &'a str) -> &mut Self {
        self.series_type = Some(series_type);
        self
    }

    /// Add extra param to builder
    pub fn extra_param<T, U>(&mut self, param: T, value: U) -> &mut Self
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        self.extra_params
            .insert(param.to_string(), value.to_string());
        self
    }

    fn create_url(&self) -> String {
        let interval_val = match self.interval {
            TechnicalIndicatorInterval::OneMin => "1min",
            TechnicalIndicatorInterval::FiveMin => "5min",
            TechnicalIndicatorInterval::FifteenMin => "15min",
            TechnicalIndicatorInterval::ThirtyMin => "30min",
            TechnicalIndicatorInterval::SixtyMin => "60min",
            TechnicalIndicatorInterval::Daily => "daily",
            TechnicalIndicatorInterval::Weekly => "weekly",
            TechnicalIndicatorInterval::Monthly => "monthly",
        };

        let mut created_link = format!(
            "query?function={}&symbol={}&interval={}",
            &self.function, &self.symbol, &interval_val
        );

        if let Some(time_period) = &self.time_period {
            created_link.push_str(&format!("&time_period={}", &time_period));
        }

        if let Some(series_type) = &self.series_type {
            created_link.push_str(&format!("&series_type={}", &series_type));
        }

        for (param, value) in &self.extra_params {
            created_link.push_str(&format!("&{}={}", &param, &value));
        }

        created_link
    }

    /// Returns JSON data struct
    ///
    /// # Errors
    /// Raise error if data obtained cannot be properly converted to struct or
    /// API returns any 4 possible known errors
    pub async fn json(&self) -> Result<TechnicalIndicator> {
        let url = self.create_url();
        let indicator_helper: TechnicalIndicatorHelper = self.api_client.get_json(&url).await?;
        indicator_helper.convert()
    }
}

/// Enum for declaring interval for technical indicator
#[derive(Clone)]
pub enum TechnicalIndicatorInterval {
    /// 1 min interval
    OneMin,
    /// 5 min interval
    FiveMin,
    /// 15 min interval
    FifteenMin,
    /// 30 min interval
    ThirtyMin,
    /// 60 min interval
    SixtyMin,
    /// daily interval
    Daily,
    /// weekly interval
    Weekly,
    /// monthly interval
    Monthly,
}
