//! Module for Forex realtime and historical data
//!
//! APIs under this section provide a wide range of data feed for realtime and
//! historical forex (FX) rates.
//!
//! You can read about [Forex][forex] API and what it returns
//! on alphavantage documentation
//!
//! [forex]: https://www.alphavantage.co/documentation/#fx

use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    deserialize::from_str,
    error::{Error, Result},
    utils::{detect_common_helper_error, ForexFunction, OutputSize, TimeSeriesInterval},
};

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
    /// use alpha_vantage::utils::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::IntraDay,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
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
    /// use alpha_vantage::utils::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::IntraDay,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
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
    ///     use alpha_vantage::utils::*;
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::IntraDay,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
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
    /// use alpha_vantage::utils::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::IntraDay,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
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
    /// use alpha_vantage::utils::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::IntraDay,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::FiveMin,
    ///             OutputSize::Full,
    ///         )
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
    /// convert [ForexHelper][ForexHelper] to [Forex][Forex]
    pub(crate) fn convert(self) -> Result<Forex> {
        let mut forex_struct = Forex::default();
        detect_common_helper_error(self.information, self.error_message, self.note)?;
        if self.meta_data.is_none() || self.forex.is_none() {
            return Err(Error::EmptyResponse);
        }
        let meta_data = self.meta_data.unwrap();
        let information = &meta_data["1. Information"];
        let from_symbol = &meta_data["2. From Symbol"];
        let to_symbol = &meta_data["3. To Symbol"];
        let last_refreshed = meta_data.get("4. Last Refreshed");
        let mut last_refreshed_value = last_refreshed.cloned();
        if last_refreshed_value.is_none() {
            let last_refreshed = meta_data.get("5. Last Refreshed");
            last_refreshed_value = last_refreshed.cloned();
        }
        let last_refreshed_value =
            last_refreshed_value.expect("Last refreshed value contains None");
        let time_zone = meta_data.get("5. Time Zone");
        let mut time_zone_value = time_zone.cloned();
        if time_zone_value.is_none() {
            let time_zone = meta_data.get("6. Time Zone");
            time_zone_value = time_zone.cloned();
        }
        if time_zone_value.is_none() {
            let time_zone = meta_data.get("7. Time Zone");
            time_zone_value = time_zone.cloned();
        }
        let time_zone_value = time_zone_value.expect("Time zone contains None value");
        let output_size = meta_data.get("4. Output Size");
        let mut output_size_value = output_size.cloned();
        if output_size_value.is_none() {
            let output_size = meta_data.get("6. Output Size");
            output_size_value = output_size.cloned();
        }
        let interval = meta_data.get("5. Interval");
        let interval_value = interval.cloned();
        forex_struct.meta_data = MetaData {
            information: information.to_string(),
            from_symbol: from_symbol.to_string(),
            to_symbol: to_symbol.to_string(),
            last_refreshed: last_refreshed_value,
            interval: interval_value,
            output_size: output_size_value,
            time_zone: time_zone_value,
        };
        let mut forex_entries: Vec<Entry> = Vec::new();
        for hash in self.forex.unwrap().values() {
            for val in hash.keys() {
                let mut entry = Entry {
                    time: val.to_string(),
                    ..Entry::default()
                };
                let entry_helper = hash
                    .get(val)
                    .expect("Cannot get a val from hash map")
                    .clone();
                entry.open = entry_helper.open;
                entry.high = entry_helper.high;
                entry.low = entry_helper.low;
                entry.close = entry_helper.close;
                forex_entries.push(entry);
            }
        }
        forex_struct.forex = forex_entries;
        Ok(forex_struct)
    }
}

/// trait which helps for performing some common operation on Vec<Entry>
pub trait VecEntry {
    /// Find a entry with a given time as a input return none if no entry found
    fn find(&self, time: &str) -> Option<Entry>;
    /// Return a entry which is of latest time period
    fn latest(&self) -> Entry;
    /// Return a top n latest Entry
    /// # Errors
    /// If n is greater than no of entry
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

/// Create Url from given user parameter for reqwest crate
pub(crate) fn create_url(
    function: ForexFunction,
    from_symbol: &str,
    to_symbol: &str,
    interval: TimeSeriesInterval,
    output_size: OutputSize,
    api: &str,
) -> String {
    let function = match function {
        ForexFunction::IntraDay => "FX_INTRADAY",
        ForexFunction::Daily => "FX_DAILY",
        ForexFunction::Weekly => "FX_WEEKLY",
        ForexFunction::Monthly => "FX_MONTHLY",
    };

    let mut url = format!(
        "query?function={}&from_symbol={}&to_symbol={}",
        function, from_symbol, to_symbol
    );
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

// Test module
#[cfg(test)]
mod test {
    use crate::utils::*;
    #[test]
    // Testing forex create_url() function
    fn test_forex_create_url() {
        assert_eq!(
            super::create_url(
                ForexFunction::Daily,
                "USD",
                "NPR",
                TimeSeriesInterval::None,
                OutputSize::None,
                "random"
            ),
            String::from("query?function=FX_DAILY&from_symbol=USD&to_symbol=NPR&apikey=random")
        );
        assert_eq!(
            super::create_url(
                ForexFunction::Weekly,
                "USD",
                "NPR",
                TimeSeriesInterval::None,
                OutputSize::None,
                "random"
            ),
            String::from("query?function=FX_WEEKLY&from_symbol=USD&to_symbol=NPR&apikey=random")
        );
        assert_eq!(
            super::create_url(
                ForexFunction::Monthly,
                "USD",
                "NPR",
                TimeSeriesInterval::None,
                OutputSize::None,
                "random"
            ),
            String::from("query?function=FX_MONTHLY&from_symbol=USD&to_symbol=NPR&apikey=random")
        );
        assert_eq!(
            super::create_url(
                ForexFunction::IntraDay,
                "USD",
                "NPR",
                TimeSeriesInterval::FifteenMin,
                OutputSize::Full,
                "random"
            ),
            String::from(
                "query?function=FX_INTRADAY&from_symbol=USD&to_symbol=NPR&interval=15min&\
                 outputsize=full&apikey=random"
            )
        );
    }
}
