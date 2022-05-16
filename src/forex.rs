//! Module for Forex realtime and historical data
//!
//! APIs under this section provide a wide range of data feed for realtime and
//! historical forex (FX) rates.
//!
//! You can read about [Forex][forex] API and what it returns
//! on alphavantage documentation
//!
//! [forex]: https://www.alphavantage.co/documentation/#fx

use std::cmp;
use std::collections::HashMap;
use std::fmt::Write;

use serde::Deserialize;

use crate::api::{ApiClient, OutputSize, TimeSeriesInterval};
use crate::deserialize::from_str;
use crate::error::{detect_common_helper_error, Error, Result};

/// Struct used to store metadata value
#[derive(Debug, Clone, Default)]
struct MetaData {
    information: String,
    from_symbol: String,
    to_symbol: String,
    last_refreshed: String,
    interval: Option<String>,
    output_size: Option<String>,
    time_zone: String,
}

/// Struct to store Entry value
#[derive(Default, Debug, Clone)]
pub struct Entry {
    time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

impl Entry {
    /// Return time for entry
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return open value
    #[must_use]
    pub fn open(&self) -> f64 {
        self.open
    }

    /// Return high value
    #[must_use]
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Return low value
    #[must_use]
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Return close value
    #[must_use]
    pub fn close(&self) -> f64 {
        self.close
    }
}

/// Struct to store Forex data after forex API call
#[derive(Debug, Default)]
pub struct Forex {
    meta_data: MetaData,
    forex: Vec<Entry>,
}

impl Forex {
    /// Return information of data
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let information = forex.information();
    ///     assert_eq!(information, "FX Intraday (5min) Time Series");
    /// }
    /// ```
    #[must_use]
    pub fn information(&self) -> &str {
        self.return_meta_string("information")
    }

    /// Return from symbol
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let symbol_from = forex.symbol_from();
    ///     assert_eq!(symbol_from, "EUR");
    /// }
    /// ```
    #[must_use]
    pub fn symbol_from(&self) -> &str {
        self.return_meta_string("from symbol")
    }

    /// Return to symbol
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let symbol_to = forex.symbol_to();
    ///     assert_eq!(symbol_to, "USD");
    /// }
    /// ```
    #[must_use]
    pub fn symbol_to(&self) -> &str {
        self.return_meta_string("to symbol")
    }

    /// Return last refreshed time
    #[must_use]
    pub fn last_refreshed(&self) -> &str {
        self.return_meta_string("last refreshed")
    }

    /// Return time zone of all data time
    #[must_use]
    pub fn time_zone(&self) -> &str {
        self.return_meta_string("time zone")
    }

    /// Return interval for intraday
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let interval = forex.interval();
    ///     assert_eq!(interval.unwrap(), "5min");
    /// }
    /// ```
    #[must_use]
    pub fn interval(&self) -> Option<&str> {
        self.operate_option_meta_value("interval")
    }

    /// Return output size which can be full or compact
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let output_size = forex.output_size();
    ///     assert_eq!(output_size.unwrap(), "Full size");
    /// }
    /// ```
    #[must_use]
    pub fn output_size(&self) -> Option<&str> {
        self.operate_option_meta_value("output size")
    }

    /// Method return Entry
    #[must_use]
    pub fn entry(&self) -> &Vec<Entry> {
        &self.forex
    }

    /// Return a meta data field
    fn return_meta_string(&self, which_val: &str) -> &str {
        match which_val {
            "information" => &self.meta_data.information,
            "from symbol" => &self.meta_data.from_symbol,
            "to symbol" => &self.meta_data.to_symbol,
            "time zone" => &self.meta_data.time_zone,
            "last refreshed" => &self.meta_data.last_refreshed,
            _ => "",
        }
    }

    /// Convert Option meta data field as a Option<&str>
    fn operate_option_meta_value(&self, which_val: &str) -> Option<&str> {
        let value = match which_val {
            "interval" => &self.meta_data.interval,
            "output size" => &self.meta_data.output_size,
            _ => &None,
        };
        value.as_deref()
    }
}

/// Entry Helper
#[derive(Clone, Debug, Deserialize)]
struct EntryHelper {
    #[serde(rename = "1. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "from_str")]
    close: f64,
}

/// struct which helps for collecting forex data from website
#[derive(Debug, Deserialize)]
pub(crate) struct ForexHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten)]
    forex: Option<HashMap<String, HashMap<String, EntryHelper>>>,
}

impl ForexHelper {
    /// convert `ForexHelper` to `Forex`
    pub(crate) fn convert(self) -> Result<Forex> {
        detect_common_helper_error(self.information, self.error_message, self.note)?;

        if self.meta_data.is_none() || self.forex.is_none() {
            return Err(Error::EmptyResponse);
        }

        let meta_data = self.meta_data.unwrap();

        let information = &meta_data["1. Information"];
        let from_symbol = &meta_data["2. From Symbol"];
        let to_symbol = &meta_data["3. To Symbol"];

        let mut last_refreshed = meta_data.get("4. Last Refreshed");
        if last_refreshed.is_none() {
            last_refreshed = meta_data.get("5. Last Refreshed");
        };

        let time_zone_value = meta_data.get("5. Time Zone").unwrap_or_else(|| {
            meta_data.get("6. Time Zone").unwrap_or_else(|| {
                meta_data
                    .get("7. Time Zone")
                    .expect("Time zone contains None value")
            })
        });

        let mut output_size_value = meta_data.get("4. Output Size");
        if output_size_value.is_none() {
            output_size_value = meta_data.get("6. Output Size");
        }

        let interval = meta_data.get("5. Interval");

        let meta_data = MetaData {
            information: information.to_string(),
            from_symbol: from_symbol.to_string(),
            to_symbol: to_symbol.to_string(),
            last_refreshed: last_refreshed
                .expect("Last refreshed value contains None")
                .to_string(),
            interval: interval.map(ToString::to_string),
            output_size: output_size_value.map(ToString::to_string),
            time_zone: time_zone_value.to_string(),
        };
        let mut forex_entries: Vec<Entry> = Vec::new();
        for hash in self.forex.unwrap().values() {
            for val in hash.keys() {
                let entry_helper = hash
                    .get(val)
                    .expect("failed to get value from Forex hashmap");

                forex_entries.push(Entry {
                    time: val.to_string(),
                    open: entry_helper.open,
                    high: entry_helper.high,
                    low: entry_helper.low,
                    close: entry_helper.close,
                });
            }
        }

        Ok(Forex {
            forex: forex_entries,
            meta_data,
        })
    }
}

/// trait which helps for performing some common operation on Vec<Entry>
pub trait VecEntry {
    /// Find a entry with a given time as a input return none if no entry found
    fn find(&self, time: &str) -> Option<&Entry>;
    /// Return a entry which is of latest time period
    fn latest(&self) -> Entry;
    /// Return a top n latest Entry
    /// # Errors
    /// If n is greater than no of entry
    fn latest_n(&self, n: usize) -> Result<Vec<&Entry>>;
}

impl VecEntry for Vec<Entry> {
    #[must_use]
    fn find(&self, time: &str) -> Option<&Entry> {
        for entry in self {
            if entry.time == time {
                return Some(entry);
            }
        }
        None
    }

    #[must_use]
    fn latest(&self) -> Entry {
        let mut latest = &Entry::default();
        for entry in self {
            if latest.time < entry.time {
                latest = entry;
            }
        }
        latest.clone()
    }

    fn latest_n(&self, n: usize) -> Result<Vec<&Entry>> {
        let mut time_list = self.iter().map(|entry| &entry.time).collect::<Vec<_>>();
        time_list.sort_by_key(|w| cmp::Reverse(*w));

        if n > time_list.len() {
            return Err(Error::DesiredNumberOfEntryNotPresent(time_list.len()));
        }

        let mut full_list = Vec::<&Entry>::new();

        for time in &time_list[0..n] {
            full_list.push(self.find(time).unwrap());
        }

        Ok(full_list)
    }
}

/// Builder to create `Forex`
pub struct ForexBuilder<'a> {
    api_client: &'a ApiClient<'a>,
    function: ForexFunction,
    from_symbol: &'a str,
    to_symbol: &'a str,
    interval: Option<TimeSeriesInterval>,
    output_size: Option<OutputSize>,
}

impl<'a> ForexBuilder<'a> {
    /// Create new `ForexBuilder` from `APIClient`
    #[must_use]
    pub fn new(
        api_client: &'a ApiClient,
        function: ForexFunction,
        from_symbol: &'a str,
        to_symbol: &'a str,
    ) -> Self {
        Self {
            api_client,
            function,
            from_symbol,
            to_symbol,
            interval: None,
            output_size: None,
        }
    }

    /// Define time series interval for forex
    #[must_use]
    pub fn interval(mut self, interval: TimeSeriesInterval) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Define output size for intraday or daily forex
    #[must_use]
    pub fn output_size(mut self, output_size: OutputSize) -> Self {
        self.output_size = Some(output_size);
        self
    }

    fn create_url(&self) -> Result<String> {
        let function = match self.function {
            ForexFunction::IntraDay => "FX_INTRADAY",
            ForexFunction::Daily => "FX_DAILY",
            ForexFunction::Weekly => "FX_WEEKLY",
            ForexFunction::Monthly => "FX_MONTHLY",
        };

        let mut url = format!(
            "query?function={}&from_symbol={}&to_symbol={}",
            function, self.from_symbol, self.to_symbol
        );

        if let Some(forex_interval) = &self.interval {
            let interval = match forex_interval {
                TimeSeriesInterval::OneMin => "1min",
                TimeSeriesInterval::FiveMin => "5min",
                TimeSeriesInterval::FifteenMin => "15min",
                TimeSeriesInterval::ThirtyMin => "30min",
                TimeSeriesInterval::SixtyMin => "60min",
            };
            write!(url, "&interval={}", interval).map_err(|_| Error::CreateUrl)?;
        };

        if let Some(forex_output_size) = &self.output_size {
            let size = match forex_output_size {
                OutputSize::Full => "full",
                OutputSize::Compact => "compact",
            };
            write!(url, "&outputsize={}", size).map_err(|_| Error::CreateUrl)?;
        }

        Ok(url)
    }

    /// Returns JSON data struct
    ///
    /// # Errors
    /// Raise error if data obtained cannot be properly converted to struct or
    /// API returns any 4 possible known errors
    pub async fn json(&self) -> Result<Forex> {
        let url = self.create_url()?;
        let forex_helper: ForexHelper = self.api_client.get_json(&url).await?;
        forex_helper.convert()
    }
}

/// Enum for declaring function for forex function by defining which type of
/// forex series to be returned
#[derive(Clone)]
pub enum ForexFunction {
    /// returns intraday time series (timestamp, open, high, low, close) of the
    /// FX currency pair specified, updated realtime
    IntraDay,
    /// returns the daily time series (timestamp, open, high, low, close) of the
    /// FX currency pair specified, updated realtime
    Daily,
    /// returns the weekly time series (timestamp, open, high, low, close) of
    /// the FX currency pair specified, updated realtime.
    Weekly,
    /// returns the monthly time series (timestamp, open, high, low, close) of
    /// the FX currency pair specified, updated realtime
    Monthly,
}
