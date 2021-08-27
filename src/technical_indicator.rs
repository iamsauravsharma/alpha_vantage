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

use crate::{
    api::ApiClient,
    error::{Error, Result},
    utils::{
        detect_common_helper_error, TechnicalIndicator as UtilIndicator, TechnicalIndicatorInterval,
    },
};

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
pub struct Indicator {
    metadata: HashMap<String, Value>,
    data: DataType,
}

impl Indicator {
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
                    .expect("cannot get time key value from hash map")
                    .clone();
                for (key, value) in &hash_values {
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

/// Struct for helping indicator struct
#[derive(Deserialize)]
pub(crate) struct IndicatorHelper {
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

impl IndicatorHelper {
    pub(crate) fn convert(self) -> Result<Indicator> {
        let mut indicator = Indicator::default();
        detect_common_helper_error(self.information, self.error_message, self.note)?;
        if self.metadata.is_none() || self.data.is_none() {
            return Err(Error::EmptyResponse);
        }
        indicator.metadata = self.metadata.unwrap();
        indicator.data = self.data.unwrap();
        Ok(indicator)
    }
}

/// Builder to help create `Indicator`
pub struct IndicatorBuilder<'a> {
    api_client: &'a ApiClient<'a>,
    function: &'a str,
    symbol: &'a str,
    interval: TechnicalIndicatorInterval,
    time_period: Option<u64>,
    series_type: Option<&'a str>,
    extras: Vec<UtilIndicator>,
}

impl<'a> IndicatorBuilder<'a> {
    /// Create new `IndicatorBuilder` form `APIClient`
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
            extras: vec![],
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

    /// Add extra params to builder
    pub fn extras_param(&mut self, extra: UtilIndicator) -> &mut Self {
        self.extras.push(extra);
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
            self.function, self.symbol, interval_val
        );
        if let Some(time_period) = self.time_period {
            created_link.push_str(&format!("&time_period={}", time_period));
        }
        if let Some(series_type) = self.series_type {
            created_link.push_str(&format!("&series_type={}", series_type));
        }
        for values in &self.extras {
            match values {
                UtilIndicator::Acceleration(val) => {
                    created_link.push_str(&format!("&acceleration={}", val));
                }
                UtilIndicator::Fastdmatype(val) => {
                    created_link.push_str(&format!("&fastdmatype={}", val));
                }
                UtilIndicator::Fastdperiod(val) => {
                    created_link.push_str(&format!("&fastdperiod={}", val));
                }
                UtilIndicator::Fastkperiod(val) => {
                    created_link.push_str(&format!("&fastkperiod={}", val));
                }
                UtilIndicator::Fastlimit(val) => {
                    created_link.push_str(&format!("&fastlimit={}", val));
                }
                UtilIndicator::Fastmatype(val) => {
                    created_link.push_str(&format!("&fastmatype={}", val));
                }
                UtilIndicator::Fastperiod(val) => {
                    created_link.push_str(&format!("&fastperiod={}", val));
                }
                UtilIndicator::Matype(val) => created_link.push_str(&format!("&matype={}", val)),
                UtilIndicator::Maximum(val) => created_link.push_str(&format!("&maximum={}", val)),
                UtilIndicator::Nbdevdn(val) => created_link.push_str(&format!("&nbdevdn={}", val)),
                UtilIndicator::Nbdevup(val) => created_link.push_str(&format!("&nbdevup={}", val)),
                UtilIndicator::Signalmatype(val) => {
                    created_link.push_str(&format!("&signalmatype={}", val));
                }
                UtilIndicator::Signalperiod(val) => {
                    created_link.push_str(&format!("&signalperiod={}", val));
                }
                UtilIndicator::Slowdmatype(val) => {
                    created_link.push_str(&format!("&slowdmatype={}", val));
                }
                UtilIndicator::Slowdperiod(val) => {
                    created_link.push_str(&format!("&slowdperiod={}", val));
                }
                UtilIndicator::Slowkmatype(val) => {
                    created_link.push_str(&format!("&slowkmatype={}", val));
                }
                UtilIndicator::Slowkperiod(val) => {
                    created_link.push_str(&format!("&slowkperiod={}", val));
                }
                UtilIndicator::Slowlimit(val) => {
                    created_link.push_str(&format!("&slowlimit={}", val));
                }
                UtilIndicator::Slowmatype(val) => {
                    created_link.push_str(&format!("&slowmatype={}", val));
                }
                UtilIndicator::Slowperiod(val) => {
                    created_link.push_str(&format!("&slowperiod={}", val));
                }
                UtilIndicator::Timeperiod1(val) => {
                    created_link.push_str(&format!("&timeperiod1={}", val));
                }
                UtilIndicator::Timeperiod2(val) => {
                    created_link.push_str(&format!("&timeperiod2={}", val));
                }
                UtilIndicator::Timeperiod3(val) => {
                    created_link.push_str(&format!("&timeperiod3={}", val));
                }
            }
        }
        created_link.push_str(&format!("&apikey={}", self.api_client.get_api_key()));
        created_link
    }

    /// Returns JSON data struct
    ///
    /// # Errors
    /// Raise error if data obtained cannot be properly converted to struct or
    /// API returns any 4 possible known errors
    pub async fn json(&self) -> Result<Indicator> {
        let url = self.create_url();
        let indicator_helper: IndicatorHelper = self.api_client.get_json(url).await?;
        indicator_helper.convert()
    }
}
