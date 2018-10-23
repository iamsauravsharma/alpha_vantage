use crate::util::{ForexFunction, Interval, OutputSize};
use reqwest::Url;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

/// Struct to store Forex data after forex API call
#[derive(Debug)]
pub struct Forex {
    error_message: Option<String>,
    information: Option<String>,
    meta_data: Option<MetaData>,
    forex: Option<Vec<Entry>>,
}

impl Forex {
    //create new forex struct
    fn new() -> Forex {
        Forex {
            error_message: None,
            information: None,
            meta_data: None,
            forex: None,
        }
    }

    ///Method return MetaData
    pub fn meta_data(&self) -> Option<MetaData> {
        self.meta_data.clone()
    }

    ///Method return Entry
    pub fn entry(&self) -> Option<Vec<Entry>> {
        self.forex.clone()
    }
}

///Struct used to store MetaData value
#[derive(Debug, Clone)]
pub struct MetaData {
    information: String,
    from_symbol: String,
    to_symbol: String,
    last_refreshed: String,
    interval: Option<String>,
    output_size: Option<String>,
    time_zone: String,
}

impl MetaData {
    /// Return last refreshed
    pub fn last_refreshed(&self) -> String {
        self.last_refreshed.clone()
    }

    /// Return time zone
    pub fn time_zone(&self) -> String {
        self.time_zone.clone()
    }
}

///Struct to store Entry value
#[derive(Default, Debug, Clone)]
pub struct Entry {
    time: String,
    open: String,
    high: String,
    low: String,
    close: String,
}

impl Entry {
    /// Return time for entry
    pub fn get_time(&self) -> String {
        self.time.clone()
    }

    /// Return open value
    pub fn get_open(&self) -> f64 {
        return_f64(self.open.clone())
    }

    /// Return high value
    pub fn get_high(&self) -> f64 {
        return_f64(self.high.clone())
    }

    /// Return low value
    pub fn get_low(&self) -> f64 {
        return_f64(self.low.clone())
    }

    /// Return close value
    pub fn get_close(&self) -> f64 {
        return_f64(self.close.clone())
    }
}

fn return_f64(data: String) -> f64 {
    data.trim().parse::<f64>().unwrap()
}

// struct which helps for collecting forex data from website
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
    //convert ForexHelper to Forex
    pub(crate) fn convert(self) -> Forex {
        let mut forex = Forex::new();
        forex.error_message = self.error_message;
        forex.information = self.information;
        if let Some(meta_data) = self.meta_data {
            let information = &meta_data["1. Information"];
            let from_symbol = &meta_data["2. From Symbol"];
            let to_symbol = &meta_data["3. To Symbol"];
            let last_refreshed = meta_data.get("4. Last Refreshed");
            let mut last_refreshed_value = return_value(last_refreshed);
            if last_refreshed_value.is_none() {
                let last_refreshed = meta_data.get("5. Last Refreshed");
                last_refreshed_value = return_value(last_refreshed);
            }
            let time_zone = meta_data.get("5. Time Zone");
            let mut time_zone_value = return_value(time_zone);
            if time_zone_value.is_none() {
                let time_zone = meta_data.get("6. Time Zone");
                time_zone_value = return_value(time_zone);
            }
            if time_zone_value.is_none() {
                let time_zone = meta_data.get("7. Time Zone");
                time_zone_value = return_value(time_zone);
            }
            let output_size = meta_data.get("4. Output Size");
            let mut output_size_value = return_value(output_size);
            if output_size_value.is_none() {
                let output_size = meta_data.get("6. Output Size");
                output_size_value = return_value(output_size);
            }
            let interval = meta_data.get("5. Interval");
            let interval_value = return_value(interval);
            forex.meta_data = Some(MetaData {
                information: information.to_string(),
                from_symbol: from_symbol.to_string(),
                to_symbol: to_symbol.to_string(),
                last_refreshed: last_refreshed_value.unwrap(),
                interval: interval_value,
                output_size: output_size_value,
                time_zone: time_zone_value.unwrap(),
            });
        }
        let mut value: Vec<Entry> = Vec::new();
        if let Some(entry) = self.forex {
            for hash in entry.values() {
                for val in hash.keys() {
                    let mut entry: Entry = Default::default();
                    entry.time = val.to_string();
                    let entry_helper = hash.get(val).unwrap().clone();
                    entry.open = entry_helper.open;
                    entry.high = entry_helper.high;
                    entry.low = entry_helper.low;
                    entry.close = entry_helper.close;
                    value.push(entry);
                }
            }
        }
        if !value.is_empty() {
            forex.forex = Some(value);
        }
        forex
    }
}

//Entry Helper
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

//Convert Option(&String) to String
fn return_value(value: Option<&std::string::String>) -> Option<String> {
    match value {
        Some(value) => Some(value.to_string()),
        None => None,
    }
}

//Create Url from given user paramter for reqwest crate 
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
        url.push_str(format!("&interval={}", interval).as_str());
    }

    url.push_str(match output_size {
        OutputSize::Full => "&outputsize=full",
        _ => "",
    });

    url.push_str(format!("&apikey={}", api).as_str());
    url.parse().unwrap()
}

//Test module
#[cfg(test)]
mod test {
    use crate::util::*;
    use reqwest::Url;
    #[test]
    //Testing forex create_url() function
    fn test_forex_create_url() {
        assert_eq!(super::create_url(ForexFunction::Daily, "USD", "NPR", Interval::None, OutputSize::None, "random"),
        Url::parse("https://www.alphavantage.co/query?function=FX_DAILY&from_symbol=USD&to_symbol=NPR&apikey=random").unwrap());
        assert_eq!(super::create_url(ForexFunction::IntraDay, "USD", "NPR", Interval::FifteenMin, OutputSize::Full, "random"),
        Url::parse("https://www.alphavantage.co/query?function=FX_INTRADAY&from_symbol=USD&to_symbol=NPR&interval=15min&outputsize=full&apikey=random").unwrap());
    }
}
