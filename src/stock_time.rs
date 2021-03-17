//! Module for stock time series
//!
//! This suite of APIs provide realtime and historical global equity data in 4
//! different temporal resolutions: (1) daily, (2) weekly, (3) monthly, and (4)
//! intraday. Daily, weekly, and monthly time series contain 20+ years of
//! historical data
//!
//! You can read about [Stock Time][stock_time] API and what it returns
//! on alphavantage documentation
//!
//! [stock_time]: https://www.alphavantage.co/documentation/#time-series-data

use std::{collections::HashMap, str::FromStr};

use serde::Deserialize;

use crate::{
    deserialize::from_str,
    error::{Error, Result},
    utils::{detect_common_helper_error, OutputSize, StockFunction, TimeSeriesInterval},
};

/// Struct for storing Meta Data value
#[derive(Debug, Clone, Default)]
pub struct MetaData {
    information: String,
    symbol: String,
    last_refreshed: String,
    interval: Option<String>,
    output_size: Option<String>,
    time_zone: String,
}

/// Struct for Entry value
#[derive(Default, Debug, Clone)]
pub struct Entry {
    time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adjusted_close: Option<f64>,
    volume: u64,
    dividend_amount: Option<f64>,
    split_coefficient: Option<f64>,
}

impl Entry {
    /// Get time
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return open
    #[must_use]
    pub fn open(&self) -> f64 {
        self.open
    }

    /// Return high
    #[must_use]
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Return low
    #[must_use]
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Return close
    #[must_use]
    pub fn close(&self) -> f64 {
        self.close
    }

    /// Return adjusted
    #[must_use]
    pub fn adjusted(&self) -> Option<f64> {
        self.adjusted_close
    }

    /// Return volume
    #[must_use]
    pub fn volume(&self) -> u64 {
        self.volume
    }

    /// Return dividend
    #[must_use]
    pub fn dividend(&self) -> Option<f64> {
        self.dividend_amount
    }

    /// Return split dividend
    #[must_use]
    pub fn split(&self) -> Option<f64> {
        self.split_coefficient
    }
}

/// Struct for storing time series data
#[derive(Debug, Default)]
pub struct TimeSeries {
    meta_data: MetaData,
    entry: Vec<Entry>,
}

impl TimeSeries {
    /// Return information present in meta data
    ///
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let stock_time = api
    ///         .stock_time(
    ///             StockFunction::IntraDay,
    ///             "MSFT",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
    ///         .await
    ///         .unwrap();
    ///     let information = stock_time.information();
    ///     assert_eq!(
    ///         information,
    ///         "Intraday (5min) open, high, low, close prices and volume"
    ///     );
    /// }
    /// ```
    #[must_use]
    pub fn information(&self) -> &str {
        self.return_meta_string("information")
    }

    /// Return symbol for which time series function is called
    ///
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let stock_time = api
    ///         .stock_time(
    ///             StockFunction::IntraDay,
    ///             "MSFT",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
    ///         .await
    ///         .unwrap();
    ///     let symbol = stock_time.symbol();
    ///     assert_eq!(symbol, "MSFT");
    /// }
    /// ```
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.return_meta_string("symbol")
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

    /// Time series interval between two consecutive data
    ///
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let stock_time = api
    ///         .stock_time(
    ///             StockFunction::IntraDay,
    ///             "MSFT",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
    ///         .await
    ///         .unwrap();
    ///     let interval = stock_time.interval();
    ///     assert_eq!(interval.unwrap(), "5min");
    /// }
    /// ```
    #[must_use]
    pub fn interval(&self) -> Option<&str> {
        self.operate_option_meta_value("interval")
    }

    /// Output Size of intraday which can be either Full or compact
    ///
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let stock_time = api
    ///         .stock_time(
    ///             StockFunction::IntraDay,
    ///             "MSFT",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
    ///         .await
    ///         .unwrap();
    ///     let output_size = stock_time.output_size();
    ///     assert_eq!(output_size.unwrap(), "Full size");
    /// }
    /// ```
    #[must_use]
    pub fn output_size(&self) -> Option<&str> {
        self.operate_option_meta_value("output size")
    }

    /// Return Entry
    #[must_use]
    pub fn entry(&self) -> &Vec<Entry> {
        &self.entry
    }

    /// Return a meta data value as a form of String
    fn return_meta_string(&self, which_val: &str) -> &str {
        match which_val {
            "information" => &self.meta_data.information,
            "symbol" => &self.meta_data.symbol,
            "time zone" => &self.meta_data.time_zone,
            "last refreshed" => &self.meta_data.last_refreshed,
            _ => "",
        }
    }

    /// Return Option metadata value as a Result form of String
    fn operate_option_meta_value(&self, which_val: &str) -> Option<&str> {
        let value = match which_val {
            "interval" => &self.meta_data.interval,
            "output size" => &self.meta_data.output_size,
            _ => &None,
        };
        value.as_deref()
    }
}

/// Helper struct to store non adjusted data
#[derive(Clone, Deserialize)]
struct EntryHelper {
    #[serde(rename = "1. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "from_str")]
    close: f64,
    #[serde(rename = "5. volume", deserialize_with = "from_str")]
    volume: u64,
}

/// Helper struct to store adjusted data
#[derive(Deserialize, Clone)]
struct AdjustedHelper {
    #[serde(rename = "1. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "from_str")]
    close: f64,
    #[serde(rename = "5. adjusted close")]
    adjusted_close: Option<String>,
    #[serde(rename = "6. volume", deserialize_with = "from_str")]
    volume: u64,
    #[serde(rename = "7. dividend amount")]
    dividend_amount: Option<String>,
    #[serde(rename = "8. split coefficient")]
    split_coefficient: Option<String>,
}

/// helper struct for `TimeSeries` which deserialize JSON
#[derive(Deserialize)]
pub(crate) struct TimeSeriesHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten)]
    time_series: Option<HashMap<String, HashMap<String, EntryHelper>>>,
    #[serde(flatten)]
    adjusted_series: Option<HashMap<String, HashMap<String, AdjustedHelper>>>,
}

impl TimeSeriesHelper {
    /// Convert [TimeSeriesHelper][TimeSeriesHelper] to [TimeSeries][TimeSeries]
    pub(crate) fn convert(self) -> Result<TimeSeries> {
        let mut time_series = TimeSeries::default();
        detect_common_helper_error(self.information, self.error_message, self.note)?;
        if self.meta_data.is_none()
            || (self.time_series.is_none() && self.adjusted_series.is_none())
        {
            return Err(Error::EmptyResponse);
        }
        let meta_data = self.meta_data.unwrap();
        let information = &meta_data["1. Information"];
        let symbol = &meta_data["2. Symbol"];
        let last_refreshed = &meta_data["3. Last Refreshed"];
        let interval = meta_data.get("4. Interval");
        let interval = interval.cloned();
        let output_size = meta_data.get("4. Output Size");
        let mut output_size_value = output_size.cloned();
        if output_size_value.is_none() {
            let output_size = meta_data.get("5. Output Size");
            output_size_value = output_size.cloned();
        }
        let time_zone = meta_data.get("4. Time Zone");
        let mut time_zone_value = time_zone.cloned();
        if time_zone_value.is_none() {
            let time_zone = meta_data.get("5. Time Zone");
            time_zone_value = time_zone.cloned()
        }
        if time_zone_value.is_none() {
            let time_zone = meta_data.get("6. Time Zone");
            time_zone_value = time_zone.cloned()
        }
        let time_zone_value = time_zone_value.expect("time zone value is None");
        time_series.meta_data = MetaData {
            information: information.to_string(),
            symbol: symbol.to_string(),
            last_refreshed: last_refreshed.to_string(),
            interval,
            output_size: output_size_value,
            time_zone: time_zone_value,
        };
        let mut entry_value: Vec<Entry> = Vec::new();
        if let Some(time_series) = self.time_series {
            for hash in time_series.values() {
                for val in hash.keys() {
                    let mut entry = Entry {
                        time: val.to_string(),
                        ..Entry::default()
                    };
                    let entry_helper = hash
                        .get(val)
                        .expect("failed to get val from hash for time series")
                        .clone();
                    entry.open = entry_helper.open;
                    entry.high = entry_helper.high;
                    entry.low = entry_helper.low;
                    entry.close = entry_helper.close;
                    entry.volume = entry_helper.volume;
                    entry_value.push(entry);
                }
            }
        }
        if let Some(adjusted_series) = self.adjusted_series {
            for hash in adjusted_series.values() {
                for val in hash.keys() {
                    let mut entry = Entry {
                        time: val.to_string(),
                        ..Entry::default()
                    };
                    let entry_helper = hash
                        .get(val)
                        .expect("failed to get val from hash for adjusted series")
                        .clone();
                    entry.open = entry_helper.open;
                    entry.high = entry_helper.high;
                    entry.low = entry_helper.low;
                    entry.close = entry_helper.close;
                    entry.volume = entry_helper.volume;
                    entry.adjusted_close = option_from_str(entry_helper.adjusted_close);
                    entry.split_coefficient = option_from_str(entry_helper.split_coefficient);
                    entry.dividend_amount = option_from_str(entry_helper.dividend_amount);
                    entry_value.push(entry);
                }
            }
        }
        time_series.entry = entry_value;
        Ok(time_series)
    }
}

/// trait which helps for performing some common operation on Vec<Entry>
pub trait VecEntry {
    /// Find a entry with a given time as a input return none if no entry found
    fn find(&self, time: &str) -> Option<Entry>;
    /// Return a entry which is of latest time period
    fn latest(&self) -> Entry;
    /// Return a top n latest Entry if n Entry is present else return Error
    fn latestn(&self, n: usize) -> Result<Vec<Entry>>;
}

impl VecEntry for Vec<Entry> {
    #[must_use]
    fn find(&self, time: &str) -> Option<Entry> {
        for entry in self {
            if entry.time == time {
                return Some(entry.clone());
            }
        }
        None
    }

    #[must_use]
    fn latest(&self) -> Entry {
        let mut latest = Entry::default();
        let mut new_time = String::new();
        for entry in self {
            if new_time < entry.time {
                latest = entry.clone();
                new_time = entry.time.clone();
            }
        }
        latest
    }

    fn latestn(&self, n: usize) -> Result<Vec<Entry>> {
        let mut time_list = Vec::new();
        for entry in self {
            time_list.push(entry.time.clone());
        }
        time_list.sort();
        time_list.reverse();
        let time_list_count: usize = time_list.len();
        let mut full_list = Self::new();
        for i in 0..n {
            let time = time_list.get(i);
            if let Some(time) = time {
                let entry = self
                    .find(time)
                    .unwrap_or_else(|| panic!("Failed to find time value for index {}", i));
                full_list.push(entry);
            } else {
                return Err(Error::DesiredNumberOfEntryNotPresent(time_list_count));
            }
        }
        Ok(full_list)
    }
}

// convert string to optional T
fn option_from_str<T>(val: Option<String>) -> Option<T>
where
    T: FromStr,
    T::Err: std::error::Error,
{
    val.map(|s| T::from_str(&s).unwrap())
}

/// create url from user provided data
pub(crate) fn create_url(
    function: StockFunction,
    symbol: &str,
    interval: TimeSeriesInterval,
    output_size: OutputSize,
    api: &str,
) -> String {
    let function = match function {
        StockFunction::IntraDay => "TIME_SERIES_INTRADAY",
        StockFunction::Daily => "TIME_SERIES_DAILY",
        StockFunction::DailyAdjusted => "TIME_SERIES_DAILY_ADJUSTED",
        StockFunction::Weekly => "TIME_SERIES_WEEKLY",
        StockFunction::WeeklyAdjusted => "TIME_SERIES_WEEKLY_ADJUSTED",
        StockFunction::Monthly => "TIME_SERIES_MONTHLY",
        StockFunction::MonthlyAdjusted => "TIME_SERIES_MONTHLY_ADJUSTED",
    };

    let mut url = format!("query?function={}&symbol={}", function, symbol);
    let interval = match interval {
        TimeSeriesInterval::OneMin => "1min",
        TimeSeriesInterval::FiveMin => "5min",
        TimeSeriesInterval::FifteenMin => "15min",
        TimeSeriesInterval::ThirtyMin => "30min",
        TimeSeriesInterval::SixtyMin => "60min",
        TimeSeriesInterval::None => "",
    };

    if !interval.is_empty() {
        url.push_str(&format!("&interval={}", interval));
    }

    url.push_str(match output_size {
        OutputSize::Full => "&outputsize=full",
        _ => "",
    });
    url.push_str(&format!("&apikey={}", api));
    url
}

#[cfg(test)]
mod test {
    use crate::utils::*;
    #[test]
    fn test_stock_time_create_url() {
        assert_eq!(
            super::create_url(
                StockFunction::Daily,
                "USD",
                TimeSeriesInterval::None,
                OutputSize::None,
                "random"
            ),
            String::from("query?function=TIME_SERIES_DAILY&symbol=USD&apikey=random")
        );
        assert_eq!(
            super::create_url(
                StockFunction::Weekly,
                "NPR",
                TimeSeriesInterval::None,
                OutputSize::None,
                "random"
            ),
            String::from("query?function=TIME_SERIES_WEEKLY&symbol=NPR&apikey=random")
        );
        assert_eq!(
            super::create_url(
                StockFunction::Monthly,
                "NPR",
                TimeSeriesInterval::None,
                OutputSize::None,
                "random"
            ),
            String::from("query?function=TIME_SERIES_MONTHLY&symbol=NPR&apikey=random")
        );
        assert_eq!(
            super::create_url(
                StockFunction::IntraDay,
                "MSFT",
                TimeSeriesInterval::SixtyMin,
                OutputSize::Full,
                "random"
            ),
            String::from(
                "query?function=TIME_SERIES_INTRADAY&symbol=MSFT&interval=60min&outputsize=full&\
                 apikey=random"
            )
        );
    }
}
