//! Module for Forex realtime and historical data
//!
//! APIs under this section provide a wide range of data feed for realtime and
//! historical forex (FX) rates.
//!
//! You can read about [Forex][forex] API and what it returns
//! on alphavantage documentation
//!
//! [forex]: https://www.alphavantage.co/documentation/#fx

use crate::{
    user::APIKey,
    util::{ForexFunction, Interval, OutputSize},
};
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

/// Struct used to store metadata value
#[derive(Debug, Clone)]
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
    open: String,
    high: String,
    low: String,
    close: String,
}

/// trait which helps for performing some common operation on Vec<Entry>
pub trait VecEntry {
    /// Find a entry with a given time as a input return none if no entry found
    fn find(&self, time: &str) -> Option<Entry>;
    /// Return a entry which is of latest time period
    fn latest(&self) -> Entry;
    /// Return a top n latest Entry if n Entry is present else return Error
    fn latestn(&self, n: usize) -> Result<Vec<Entry>, &str>;
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

    fn latestn(&self, n: usize) -> Result<Vec<Entry>, &str> {
        let mut time_list = vec![];
        for entry in self {
            time_list.push(entry.time.clone());
        }
        time_list.sort();
        time_list.reverse();
        let mut full_list = Self::new();
        for i in 0..n {
            let time = time_list.get(i);
            if let Some(time) = time {
                let entry = self
                    .find(time)
                    .expect("fail to find time value for latestn forex");
                full_list.push(entry);
            } else {
                return Err("desired number of latest Entry not found try using less value");
            }
        }
        Ok(full_list)
    }
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
        return_f64(&self.open)
    }

    /// Return high value
    #[must_use]
    pub fn high(&self) -> f64 {
        return_f64(&self.high)
    }

    /// Return low value
    #[must_use]
    pub fn low(&self) -> f64 {
        return_f64(&self.low)
    }

    /// Return close value
    #[must_use]
    pub fn close(&self) -> f64 {
        return_f64(&self.close)
    }
}

/// Return f64 from &str
fn return_f64(data: &str) -> f64 {
    data.trim()
        .parse::<f64>()
        .expect("Cannot convert string to f64")
}

/// Struct to store Forex data after forex API call
#[derive(Debug, Default)]
pub struct Forex {
    error_message: Option<String>,
    information: Option<String>,
    meta_data: Option<MetaData>,
    forex: Option<Vec<Entry>>,
}

impl Forex {
    /// Return information of data
    ///
    /// ```
    /// use alpha_vantage::util::*;
    /// let api = alpha_vantage::set_api("demo");
    /// let forex = api.forex(
    ///     ForexFunction::IntraDay,
    ///     "EUR",
    ///     "USD",
    ///     Interval::FiveMin,
    ///     OutputSize::Full,
    /// );
    /// let information = forex.information();
    /// assert_eq!(information.unwrap(), "FX Intraday (5min) Time Series");
    /// ```
    pub fn information(&self) -> Result<&str, &str> {
        self.return_meta_string("information")
    }

    /// Return from symbol
    ///
    /// ```
    /// use alpha_vantage::util::*;
    /// let api = alpha_vantage::set_api("demo");
    /// let forex = api.forex(
    ///     ForexFunction::IntraDay,
    ///     "EUR",
    ///     "USD",
    ///     Interval::FiveMin,
    ///     OutputSize::Full,
    /// );
    /// let symbol_from = forex.symbol_from();
    /// assert_eq!(symbol_from.unwrap(), "EUR");
    /// ```
    pub fn symbol_from(&self) -> Result<&str, &str> {
        self.return_meta_string("from symbol")
    }

    /// Return to symbol
    ///
    /// ```
    /// use alpha_vantage::util::*;
    /// let api = alpha_vantage::set_api("demo");
    /// let forex = api.forex(
    ///     ForexFunction::IntraDay,
    ///     "EUR",
    ///     "USD",
    ///     Interval::FiveMin,
    ///     OutputSize::Full,
    /// );
    /// let symbol_to = forex.symbol_to();
    /// assert_eq!(symbol_to.unwrap(), "USD");
    /// ```
    pub fn symbol_to(&self) -> Result<&str, &str> {
        self.return_meta_string("to symbol")
    }

    /// Return last refreshed time produce error if API returns error message or
    /// information instead of meta data
    pub fn last_refreshed(&self) -> Result<&str, &str> {
        self.return_meta_string("last refreshed")
    }

    /// Return time zone of all data time produce error if API return
    /// error message or information instead of meta data
    pub fn time_zone(&self) -> Result<&str, &str> {
        self.return_meta_string("time zone")
    }

    /// Return out interval for intraday
    ///
    /// ```
    /// use alpha_vantage::util::*;
    /// let api = alpha_vantage::set_api("demo");
    /// let forex = api.forex(
    ///     ForexFunction::IntraDay,
    ///     "EUR",
    ///     "USD",
    ///     Interval::FiveMin,
    ///     OutputSize::Full,
    /// );
    /// let interval = forex.interval();
    /// assert_eq!(interval.unwrap(), "5min");
    /// ```
    pub fn interval(&self) -> Result<&str, &str> {
        self.operate_option_meta_value("interval")
    }

    /// Return output size which can be full or compact
    ///
    /// ```
    /// use alpha_vantage::util::*;
    /// let api = alpha_vantage::set_api("demo");
    /// let forex = api.forex(
    ///     ForexFunction::IntraDay,
    ///     "EUR",
    ///     "USD",
    ///     Interval::FiveMin,
    ///     OutputSize::Full,
    /// );
    /// let output_size = forex.output_size();
    /// assert_eq!(output_size.unwrap(), "Full size");
    /// ```
    pub fn output_size(&self) -> Result<&str, &str> {
        self.operate_option_meta_value("output size")
    }

    /// Method return Entry
    pub fn entry(&self) -> Result<Vec<Entry>, &str> {
        if let Some(entry) = &self.forex {
            Ok(entry.to_vec())
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }

    /// Return a meta data field in Result type
    fn return_meta_string(&self, which_val: &str) -> Result<&str, &str> {
        if let Some(meta_data) = &self.meta_data {
            let value = match which_val {
                "information" => &meta_data.information,
                "from symbol" => &meta_data.from_symbol,
                "to symbol" => &meta_data.to_symbol,
                "time zone" => &meta_data.time_zone,
                "last refreshed" => &meta_data.last_refreshed,
                _ => "",
            };
            Ok(value)
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }

    /// Convert out Option meta data field as a Result field
    fn operate_option_meta_value(&self, which_val: &str) -> Result<&str, &str> {
        if let Some(meta_data) = &self.meta_data {
            if let Some(value) = match which_val {
                "interval" => &meta_data.interval,
                "output size" => &meta_data.output_size,
                _ => &None,
            } {
                Ok(value)
            } else {
                Err("No value present")
            }
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }
}

/// Entry Helper
#[derive(Clone, Debug, Deserialize)]
struct EntryHelper {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
}

/// struct which helps for collecting forex data from website
#[derive(Debug, Deserialize)]
pub(crate) struct ForexHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten)]
    forex: Option<HashMap<String, HashMap<String, EntryHelper>>>,
}

impl ForexHelper {
    /// convert [ForexHelper][ForexHelper] to [Forex][Forex]
    pub(crate) fn convert(self) -> Forex {
        let mut forex_struct = Forex::default();
        forex_struct.error_message = self.error_message;
        forex_struct.information = self.information;
        if let Some(meta_data) = self.meta_data {
            let information = &meta_data["1. Information"];
            let from_symbol = &meta_data["2. From Symbol"];
            let to_symbol = &meta_data["3. To Symbol"];
            let last_refreshed = meta_data.get("4. Last Refreshed");
            let mut last_refreshed_value = return_option_value(last_refreshed);
            if last_refreshed_value.is_none() {
                let last_refreshed = meta_data.get("5. Last Refreshed");
                last_refreshed_value = return_option_value(last_refreshed);
            }
            let time_zone = meta_data.get("5. Time Zone");
            let mut time_zone_value = return_option_value(time_zone);
            if time_zone_value.is_none() {
                let time_zone = meta_data.get("6. Time Zone");
                time_zone_value = return_option_value(time_zone);
            }
            if time_zone_value.is_none() {
                let time_zone = meta_data.get("7. Time Zone");
                time_zone_value = return_option_value(time_zone);
            }
            let output_size = meta_data.get("4. Output Size");
            let mut output_size_value = return_option_value(output_size);
            if output_size_value.is_none() {
                let output_size = meta_data.get("6. Output Size");
                output_size_value = return_option_value(output_size);
            }
            let interval = meta_data.get("5. Interval");
            let interval_value = return_option_value(interval);
            forex_struct.meta_data = Some(MetaData {
                information: information.to_string(),
                from_symbol: from_symbol.to_string(),
                to_symbol: to_symbol.to_string(),
                last_refreshed: last_refreshed_value.expect("Last refreshed value contains None"),
                interval: interval_value,
                output_size: output_size_value,
                time_zone: time_zone_value.expect("Time zone contains None value"),
            });
        }
        let mut value: Vec<Entry> = Vec::new();
        if let Some(entry) = self.forex {
            for hash in entry.values() {
                for val in hash.keys() {
                    let mut entry: Entry = crate::forex::Entry::default();
                    entry.time = val.to_string();
                    let entry_helper = hash
                        .get(val)
                        .expect("Cannot get a val from hash map")
                        .clone();
                    entry.open = entry_helper.open;
                    entry.high = entry_helper.high;
                    entry.low = entry_helper.low;
                    entry.close = entry_helper.close;
                    value.push(entry);
                }
            }
        }
        if !value.is_empty() {
            forex_struct.forex = Some(value);
        }
        forex_struct
    }
}

/// Convert Option(&String) to String
fn return_option_value(value: Option<&std::string::String>) -> Option<String> {
    match value {
        Some(value) => Some(value.to_string()),
        None => None,
    }
}

/// Function used to create a [Forex][Forex] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
#[must_use]
pub fn forex(
    function: ForexFunction,
    from_symbol: &str,
    to_symbol: &str,
    interval: Interval,
    output_size: OutputSize,
    api_data: (&str, Option<u64>),
) -> Forex {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.forex(function, from_symbol, to_symbol, interval, output_size)
}

/// Create Url from given user parameter for reqwest crate
pub(crate) fn create_url(
    function: ForexFunction,
    from_symbol: &str,
    to_symbol: &str,
    interval: Interval,
    output_size: OutputSize,
    api: &str,
) -> Url {
    let function = match function {
        ForexFunction::IntraDay => "FX_INTRADAY",
        ForexFunction::Daily => "FX_DAILY",
        ForexFunction::Weekly => "FX_WEEKLY",
        ForexFunction::Monthly => "FX_MONTHLY",
    };

    let mut url = format!(
        "{}{}&from_symbol={}&to_symbol={}",
        LINK, function, from_symbol, to_symbol
    );
    let interval = match interval {
        Interval::OneMin => "1min",
        Interval::FiveMin => "5min",
        Interval::FifteenMin => "15min",
        Interval::ThirtyMin => "30min",
        Interval::SixtyMin => "60min",
        Interval::None => "",
    };

    if interval != "" {
        url.push_str(&format!("&interval={}", interval));
    }

    url.push_str(match output_size {
        OutputSize::Full => "&outputsize=full",
        _ => "",
    });

    url.push_str(&format!("&apikey={}", api));
    url.parse().expect("Fail to parse url")
}

// Test module
#[cfg(test)]
mod test {
    use crate::util::*;
    use reqwest::Url;
    #[test]
    // Testing forex create_url() function
    fn test_forex_create_url() {
        assert_eq!(
            super::create_url(
                ForexFunction::Daily,
                "USD",
                "NPR",
                Interval::None,
                OutputSize::None,
                "random"
            ),
            Url::parse(
                "https://www.alphavantage.co/query?function=FX_DAILY\
                 &from_symbol=USD\
                 &to_symbol=NPR\
                 &apikey=random"
            )
            .unwrap()
        );
        assert_eq!(
            super::create_url(
                ForexFunction::Weekly,
                "USD",
                "NPR",
                Interval::None,
                OutputSize::None,
                "random"
            ),
            Url::parse(
                "https://www.alphavantage.co/query?function=FX_WEEKLY\
                 &from_symbol=USD\
                 &to_symbol=NPR\
                 &apikey=random"
            )
            .unwrap()
        );
        assert_eq!(
            super::create_url(
                ForexFunction::Monthly,
                "USD",
                "NPR",
                Interval::None,
                OutputSize::None,
                "random"
            ),
            Url::parse(
                "https://www.alphavantage.co/query?function=FX_MONTHLY\
                 &from_symbol=USD\
                 &to_symbol=NPR\
                 &apikey=random"
            )
            .unwrap()
        );
        assert_eq!(
            super::create_url(
                ForexFunction::IntraDay,
                "USD",
                "NPR",
                Interval::FifteenMin,
                OutputSize::Full,
                "random"
            ),
            Url::parse(
                "https://www.alphavantage.co/query?function=FX_INTRADAY\
                 &from_symbol=USD\
                 &to_symbol=NPR\
                 &interval=15min\
                 &outputsize=full\
                 &apikey=random"
            )
            .unwrap()
        );
    }
}
