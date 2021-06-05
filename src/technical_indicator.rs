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

/// Create url for reqwest
pub(crate) fn create_url(
    function: &str,
    symbol: &str,
    interval: TechnicalIndicatorInterval,
    time_period: Option<u64>,
    series_type: Option<&str>,
    extras: Vec<UtilIndicator>,
    apikey: &str,
) -> String {
    let interval_val = match interval {
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
        function, symbol, interval_val
    );
    if let Some(time_period) = time_period {
        created_link.push_str(&format!("&time_period={}", time_period));
    }
    if let Some(series_type) = series_type {
        created_link.push_str(&format!("&series_type={}", series_type));
    }
    for values in extras {
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
            UtilIndicator::Fastlimit(val) => created_link.push_str(&format!("&fastlimit={}", val)),
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
            UtilIndicator::Slowlimit(val) => created_link.push_str(&format!("&slowlimit={}", val)),
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
    created_link.push_str(&format!("&apikey={}", apikey));
    created_link
}
