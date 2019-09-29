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

use crate::{user::APIKey, util::TechnicalIndicator as UtilIndicator};
use reqwest::Url;
use serde_derive::Deserialize;
use std::collections::HashMap;

type DataType = Option<HashMap<String, HashMap<String, HashMap<String, String>>>>;

/// Struct for indicator
#[derive(Deserialize)]
pub struct Indicator {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Meta Data")]
    metadata: Option<HashMap<String, String>>,
    #[serde(flatten)]
    data: DataType,
}

impl Indicator {
    /// Return out meta data in hash form
    pub fn meta_data(&self) -> Result<&HashMap<String, String>, &str> {
        if let Some(meta_data) = &self.metadata {
            Ok(&meta_data)
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }

    /// Return data as a vector inside result
    pub fn data(&self) -> Result<Vec<DataCollector>, &str> {
        if let Some(data) = &self.data {
            let mut vector = Vec::new();
            for hash in data.values() {
                for time in hash.keys() {
                    let mut data_collector = DataCollector::default();
                    data_collector.time = time.to_string();
                    let hash_values = hash
                        .get(time)
                        .expect("cannot get out time key value from hash map")
                        .to_owned();
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
            Ok(vector)
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }
}

/// Struct for storing a data values
#[derive(Default)]
pub struct DataCollector {
    time: String,
    values: HashMap<String, f64>,
}

impl DataCollector {
    /// Return out a time
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return values for Data
    pub fn values(&self) -> &HashMap<String, f64> {
        &self.values
    }
}

/// Function used to create a [Indicator][Indicator] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub fn technical_indicator(
    function: &str,
    symbol: &str,
    interval: &str,
    series_type: Option<&str>,
    time_period: Option<&str>,
    temporary_value: Vec<UtilIndicator>,
    api_data: (&str, Option<u64>),
) -> Indicator {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.technical_indicator(
        function,
        symbol,
        interval,
        series_type,
        time_period,
        temporary_value,
    )
}

/// Create url for reqwest
pub(crate) fn create_url(
    function: &str,
    symbol: &str,
    interval: &str,
    series_type: Option<&str>,
    time_period: Option<&str>,
    temporary_value: Vec<UtilIndicator>,
    apikey: &str,
) -> Url {
    let mut created_link = format!(
        "https://www.alphavantage.co/query?function={}&symbol={}&interval={}&apikey={}",
        function, symbol, interval, apikey
    );
    if let Some(series_type) = series_type {
        created_link.push_str(&format!("&series_type={}", series_type));
    }
    if let Some(time_period) = time_period {
        created_link.push_str(&format!("&time_period={}", time_period));
    }
    for values in temporary_value {
        match values {
            UtilIndicator::Acceleration(val) => {
                created_link.push_str(&format!("&acceleration={}", val))
            }
            UtilIndicator::Fastdmatype(val) => {
                created_link.push_str(&format!("&fastdmatype={}", val))
            }
            UtilIndicator::Fastdperiod(val) => {
                created_link.push_str(&format!("&fastdperiod={}", val))
            }
            UtilIndicator::Fastkperiod(val) => {
                created_link.push_str(&format!("&fastkperiod={}", val))
            }
            UtilIndicator::Fastlimit(val) => created_link.push_str(&format!("&fastlimit={}", val)),
            UtilIndicator::Fastmatype(val) => {
                created_link.push_str(&format!("&fastmatype={}", val))
            }
            UtilIndicator::Fastperiod(val) => {
                created_link.push_str(&format!("&fastperiod={}", val))
            }
            UtilIndicator::Matype(val) => created_link.push_str(&format!("&matype={}", val)),
            UtilIndicator::Maximum(val) => created_link.push_str(&format!("&maximum={}", val)),
            UtilIndicator::Nbdevdn(val) => created_link.push_str(&format!("&nbdevdn={}", val)),
            UtilIndicator::Nbdevup(val) => created_link.push_str(&format!("&nbdevup={}", val)),
            UtilIndicator::Signalmatype(val) => {
                created_link.push_str(&format!("&signalmatype={}", val))
            }
            UtilIndicator::Signalperiod(val) => {
                created_link.push_str(&format!("&signalperiod={}", val))
            }
            UtilIndicator::Slowdmatype(val) => {
                created_link.push_str(&format!("&slowdmatype={}", val))
            }
            UtilIndicator::Slowdperiod(val) => {
                created_link.push_str(&format!("&slowdperiod={}", val))
            }
            UtilIndicator::Slowkmatype(val) => {
                created_link.push_str(&format!("&slowkmatype={}", val))
            }
            UtilIndicator::Slowkperiod(val) => {
                created_link.push_str(&format!("&slowkperiod={}", val))
            }
            UtilIndicator::Slowlimit(val) => created_link.push_str(&format!("&slowlimit={}", val)),
            UtilIndicator::Slowmatype(val) => {
                created_link.push_str(&format!("&slowmatype={}", val))
            }
            UtilIndicator::Slowperiod(val) => {
                created_link.push_str(&format!("&slowperiod={}", val))
            }
            UtilIndicator::Timeperiod1(val) => {
                created_link.push_str(&format!("&timeperiod1={}", val))
            }
            UtilIndicator::Timeperiod2(val) => {
                created_link.push_str(&format!("&timeperiod2={}", val))
            }
            UtilIndicator::Timeperiod3(val) => {
                created_link.push_str(&format!("&timeperiod3={}", val))
            }
        }
    }
    created_link
        .parse()
        .expect("Cannot parse out created link ti url")
}
