use crate::util::*;
use reqwest::Url;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

#[derive(Debug, Deserialize)]
pub struct TimeSeriesHelper {
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
    adjusted_close: String,
    #[serde(rename = "6. volume")]
    volume: String,
    #[serde(rename = "7. dividend amount")]
    dividend_amount: String,
    #[serde(rename = "8. split coefficient")]
    split_coefficient: Option<String>,
}

pub fn create_url(
    function: Function,
    symbol: &str,
    interval: Option<Interval>,
    output_size: Option<OutputSize>,
    api: String,
) -> Url {
    let function = match function {
        Function::IntraDay => "TIME_SERIES_INTRADAY",
        Function::Daily => "TIME_SERIES_DAILY",
        Function::DailyAdjusted => "TIME_SERIES_DAILY_ADJUSTED",
        Function::Weekly => "TIME_SERIES_WEEKLY",
        Function::WeeklyAdjusted => "TIME_SERIES_WEEKLY_ADJUSTED",
        Function::Monthly => "TIME_SERIES_MONTHLY",
        Function::MonthlyAdjusted => "TIME_SERIES_MONTHLY_ADJUSTED",
    };

    let mut url = String::from(format!("{}{}&symbol={}", LINK, function, symbol));
    let interval = match interval {
        Some(interval) => match interval {
            Interval::OneMin => "1min",
            Interval::FiveMin => "5min",
            Interval::FifteenMin => "15min",
            Interval::ThirtyMin => "30min",
            Interval::SixtyMin => "60min",
        },
        None => "",
    };

    if interval != "" {
        url.push_str(format!("&interval={}", interval).as_str());
    }

    url.push_str(match output_size {
        Some(OutputSize::Full) => "&outputsize=full",
        _ => "",
    });

    url.push_str(format!("&apikey={}", api).as_str());
    url.parse().unwrap()
}
