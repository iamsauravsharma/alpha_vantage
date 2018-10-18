use crate::util::{Interval, OutputSize, StockFunction};
use reqwest::Url;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

#[derive(Debug)]
pub struct TimeSeries {
    error_message: Option<String>,
    information: Option<String>,
    meta_data: Option<MetaData>,
    entry: Option<Vec<Entry>>,
}

impl TimeSeries {
    fn new() -> TimeSeries {
        TimeSeries {
            error_message: None,
            information: None,
            meta_data: None,
            entry: None,
        }
    }
}

#[derive(Debug)]
struct MetaData {
    information: String,
    symbol: String,
    last_refreshed: String,
    interval: Option<String>,
    output_size: Option<String>,
    time_zone: String,
}

#[derive(Default, Debug)]
struct Entry {
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
    pub(crate) fn convert(self) -> TimeSeries {
        let mut time_series = TimeSeries::new();
        time_series.error_message = self.error_message;
        time_series.information = self.information;
        if let Some(meta_data) = self.meta_data {
            let information = meta_data.get("1. Information").unwrap().clone();
            let symbol = meta_data.get("2. Symbol").unwrap().clone();
            let last_refreshed = meta_data.get("3. Last Refreshed").unwrap().clone();
            let interval = meta_data.get("4. Interval");
            let interval = return_value(interval);
            let output_size = meta_data.get("4. Output Size");
            let mut output_size_value = return_value(output_size);
            if let None = output_size_value {
                let output_size = meta_data.get("5. Output Size");
                output_size_value = return_value(output_size);
            }
            let time_zone = meta_data.get("4. Time Zone");
            let mut time_zone_value = return_value(time_zone);
            if let None = time_zone_value {
                let time_zone = meta_data.get("5. Time Zone");
                time_zone_value = return_value(time_zone)
            }
            if let None = time_zone_value {
                let time_zone = meta_data.get("6. Time Zone");
                time_zone_value = return_value(time_zone)
            }
            time_series.meta_data = Some(MetaData {
                information,
                symbol,
                last_refreshed,
                interval,
                output_size: output_size_value,
                time_zone: time_zone_value.unwrap(),
            });
        }
        let mut value: Vec<Entry> = Vec::new();
        if let Some(entry) = self.time_series {
            for hash in entry.values() {
                for val in hash.keys() {
                    let mut entry: Entry = Default::default();
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
                    let mut entry: Entry = Default::default();
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

fn return_value(value: Option<&std::string::String>) -> Option<String> {
    match value {
        Some(value) => Some(value.to_string()),
        None => None,
    }
}

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

pub fn create_url(
    function: StockFunction,
    symbol: &str,
    interval: Interval,
    output_size: OutputSize,
    api: String,
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

    let mut url = String::from(format!("{}{}&symbol={}", LINK, function, symbol));
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
