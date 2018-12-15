use crate::util::{Interval, OutputSize, StockFunction};
use reqwest::Url;
use serde_derive::Deserialize;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

/// Struct for storing time series data
#[derive(Debug, Default)]
pub struct TimeSeries {
    error_message: Option<String>,
    information: Option<String>,
    meta_data: Option<MetaData>,
    entry: Option<Vec<Entry>>,
}

impl TimeSeries {
    /// Return information present in meta data
    pub fn information(&self) -> Result<String, String> {
        self.return_meta_string("information")
    }

    /// Return symbol for which time series function is called
    pub fn symbol(&self) -> Result<String, String> {
        self.return_meta_string("symbol")
    }

    /// last time a data was refreshed
    pub fn last_refreshed(&self) -> Result<String, String> {
        self.return_meta_string("last refreshed")
    }

    /// time zone of last refreshed time
    pub fn time_zone(&self) -> Result<String, String> {
        self.return_meta_string("time zone")
    }

    /// Interval for which a time series intraday
    pub fn interval(&self) -> Result<String, String> {
        self.operate_option_meta_value("interval")
    }

    /// Output Size of intraday which can be either Full or compact
    pub fn output_size(&self) -> Result<String, String> {
        self.operate_option_meta_value("output size")
    }

    /// Return Entry
    pub fn entry(&self) -> Result<Vec<Entry>, String> {
        if let Some(entry) = &self.entry {
            Ok(entry.to_vec())
        } else if let Some(error) = &self.error_message {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    /// Return a meta data value as a form of String
    fn return_meta_string(&self, which_val: &str) -> Result<String, String> {
        if let Some(meta_data) = &self.meta_data {
            let value = match which_val {
                "information" => &meta_data.information,
                "symbol" => &meta_data.symbol,
                "last refreshed" => &meta_data.last_refreshed,
                "time zone" => &meta_data.time_zone,
                _ => "",
            };
            Ok(value.to_string())
        } else if let Some(error) = &self.error_message {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    /// Return Option metadata value as a Result form of String
    fn operate_option_meta_value(&self, which_val: &str) -> Result<String, String> {
        if let Some(meta_data) = &self.meta_data {
            if let Some(value) = match which_val {
                "interval" => &meta_data.interval,
                "output size" => &meta_data.output_size,
                _ => &None,
            } {
                Ok(value.to_string())
            } else {
                Err("No value present".to_string())
            }
        } else if let Some(error) = &self.error_message {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }
}

/// Struct for storing Meta Data value
#[derive(Debug, Clone)]
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
    open: String,
    high: String,
    low: String,
    close: String,
    adjusted_close: Option<String>,
    volume: String,
    dividend_amount: Option<String>,
    split_coefficient: Option<String>,
}

impl Entry {
    /// Get time
    pub fn time(&self) -> String {
        self.time.to_string()
    }

    /// Return open
    pub fn open(&self) -> f64 {
        return_f64(self.open.as_str())
    }

    /// Return high
    pub fn high(&self) -> f64 {
        return_f64(self.high.as_str())
    }

    /// Return low
    pub fn low(&self) -> f64 {
        return_f64(self.low.as_str())
    }

    /// Return close
    pub fn close(&self) -> f64 {
        return_f64(self.close.as_str())
    }

    /// Return adjusted
    pub fn adjusted(&self) -> Option<f64> {
        if let Some(data) = self.adjusted_close.clone() {
            return Some(return_f64(&data));
        }
        None
    }

    /// Return volume
    pub fn volume(&self) -> f64 {
        return_f64(self.volume.as_str())
    }

    /// Return dividend
    pub fn dividend(&self) -> Option<f64> {
        if let Some(data) = self.dividend_amount.clone() {
            return Some(return_f64(&data));
        }
        None
    }

    /// Return split dividend
    pub fn split(&self) -> Option<f64> {
        if let Some(data) = self.split_coefficient.clone() {
            return Some(return_f64(&data));
        }
        None
    }
}

/// parse String to f64 and return value
fn return_f64(data: &str) -> f64 {
    data.trim().parse::<f64>().unwrap()
}

/// Helper struct to store non adjusted data
#[derive(Clone, Deserialize)]
struct EntryHelper {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

/// Helper struct to store adjusted data
#[derive(Deserialize, Clone)]
struct AdjustedHelper {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. adjusted close")]
    adjusted_close: Option<String>,
    #[serde(rename = "6. volume")]
    volume: String,
    #[serde(rename = "7. dividend amount")]
    dividend_amount: Option<String>,
    #[serde(rename = "8. split coefficient")]
    split_coefficient: Option<String>,
}

/// helper struct for TimeSeries which deseialize JSON
#[derive(Deserialize)]
pub(crate) struct TimeSeriesHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten)]
    time_series: Option<HashMap<String, HashMap<String, EntryHelper>>>,
    #[serde(flatten)]
    adjusted_series: Option<HashMap<String, HashMap<String, AdjustedHelper>>>,
}

impl TimeSeriesHelper {
    /// Convert TimeSeriesHelper to TimeSeries
    pub(crate) fn convert(self) -> TimeSeries {
        let mut time_series = TimeSeries::default();
        time_series.error_message = self.error_message;
        time_series.information = self.information;
        if let Some(meta_data) = self.meta_data {
            let information = &meta_data["1. Information"];
            let symbol = &meta_data["2. Symbol"];
            let last_refreshed = &meta_data["3. Last Refreshed"];
            let interval = meta_data.get("4. Interval");
            let interval = return_value(interval);
            let output_size = meta_data.get("4. Output Size");
            let mut output_size_value = return_value(output_size);
            if output_size_value.is_none() {
                let output_size = meta_data.get("5. Output Size");
                output_size_value = return_value(output_size);
            }
            let time_zone = meta_data.get("4. Time Zone");
            let mut time_zone_value = return_value(time_zone);
            if time_zone_value.is_none() {
                let time_zone = meta_data.get("5. Time Zone");
                time_zone_value = return_value(time_zone)
            }
            if time_zone_value.is_none() {
                let time_zone = meta_data.get("6. Time Zone");
                time_zone_value = return_value(time_zone)
            }
            time_series.meta_data = Some(MetaData {
                information: information.to_string(),
                symbol: symbol.to_string(),
                last_refreshed: last_refreshed.to_string(),
                interval,
                output_size: output_size_value,
                time_zone: time_zone_value.unwrap(),
            });
        }
        let mut value: Vec<Entry> = Vec::new();
        if let Some(entry) = self.time_series {
            for hash in entry.values() {
                for val in hash.keys() {
                    let mut entry: Entry = crate::time_series::Entry::default();
                    entry.time = val.to_string();
                    let entry_helper = hash.get(val).unwrap().clone();
                    entry.open = entry_helper.open;
                    entry.high = entry_helper.high;
                    entry.low = entry_helper.low;
                    entry.close = entry_helper.close;
                    entry.volume = entry_helper.volume;
                    value.push(entry);
                }
            }
        }
        if let Some(entry) = self.adjusted_series {
            for hash in entry.values() {
                for val in hash.keys() {
                    let mut entry: Entry = crate::time_series::Entry::default();
                    entry.time = val.to_string();
                    let entry_helper = hash.get(val).unwrap().clone();
                    entry.open = entry_helper.open;
                    entry.high = entry_helper.high;
                    entry.low = entry_helper.low;
                    entry.close = entry_helper.close;
                    entry.volume = entry_helper.volume;
                    entry.adjusted_close = entry_helper.adjusted_close;
                    entry.split_coefficient = entry_helper.split_coefficient;
                    entry.dividend_amount = entry_helper.dividend_amount;
                    value.push(entry);
                }
            }
        }
        if !value.is_empty() {
            time_series.entry = Some(value);
        }
        time_series
    }
}

/// Convert Option<&String> to Option<String>
fn return_value(value: Option<&std::string::String>) -> Option<String> {
    match value {
        Some(value) => Some(value.to_string()),
        None => None,
    }
}

/// create url from user provided data
pub(crate) fn create_url(
    function: StockFunction,
    symbol: &str,
    interval: Interval,
    output_size: OutputSize,
    api: &str,
) -> Url {
    let function = match function {
        StockFunction::IntraDay => "TIME_SERIES_INTRADAY",
        StockFunction::Daily => "TIME_SERIES_DAILY",
        StockFunction::DailyAdjusted => "TIME_SERIES_DAILY_ADJUSTED",
        StockFunction::Weekly => "TIME_SERIES_WEEKLY",
        StockFunction::WeeklyAdjusted => "TIME_SERIES_WEEKLY_ADJUSTED",
        StockFunction::Monthly => "TIME_SERIES_MONTHLY",
        StockFunction::MonthlyAdjusted => "TIME_SERIES_MONTHLY_ADJUSTED",
    };

    let mut url = format!("{}{}&symbol={}", LINK, function, symbol);
    let interval = match interval {
        Interval::OneMin => "1min",
        Interval::FiveMin => "5min",
        Interval::FifteenMin => "15min",
        Interval::ThirtyMin => "30min",
        Interval::SixtyMin => "60min",
        Interval::None => "",
    };

    if interval != "" {
        url.push_str(format!("&interval={}", interval).as_str());
    }

    url.push_str(match output_size {
        OutputSize::Full => "&outputsize=full",
        _ => "",
    });
    url.push_str(format!("&apikey={}", api).as_str());
    url.parse().unwrap()
}

#[cfg(test)]
mod test {
    use crate::util::*;
    use reqwest::Url;
    #[test]
    fn test_time_series_create_url() {
        assert_eq!(super::create_url(StockFunction::Daily, "USD", Interval::None, OutputSize::None, "random"),
        Url::parse("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=USD&apikey=random").unwrap());
        assert_eq!(super::create_url(StockFunction::Weekly, "NPR", Interval::None, OutputSize::None, "random"),
        Url::parse("https://www.alphavantage.co/query?function=TIME_SERIES_WEEKLY&symbol=NPR&apikey=random").unwrap());
        assert_eq!(super::create_url(StockFunction::Monthly, "NPR", Interval::None, OutputSize::None, "random"),
        Url::parse("https://www.alphavantage.co/query?function=TIME_SERIES_MONTHLY&symbol=NPR&apikey=random").unwrap());
        assert_eq!(super::create_url(StockFunction::IntraDay, "MSFT", Interval::SixtyMin, OutputSize::Full, "random"),
        Url::parse("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=MSFT&interval=60min&outputsize=full&apikey=random").unwrap());
    }
}
