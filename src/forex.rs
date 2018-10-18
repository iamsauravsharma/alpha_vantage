use crate::util::{ForexFunction,Interval,OutputSize};
use reqwest::Url;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

#[derive(Debug)]
pub struct Forex{
    error_message  : Option<String>,
    information : Option<String>,
    meta_data : Option<MetaData>,
    forex : Option<Vec<Entry>>,
}

impl Forex{
    fn new() -> Forex{
        Forex{
            error_message : None,
            information : None,
            meta_data : None,
            forex : None
        }
    }
}


#[derive(Debug)]
struct MetaData{
    information : String,
    from_symbol : String,
    to_symbol : String,
    last_refreshed : String,
    interval : Option<String>,
    output_size : Option<String>,
    time_zone : String,
}

#[derive(Default,Debug)]
struct Entry{
    time : String,
    open  :String,
    high : String,
    low : String,
    close : String,  
}

#[derive(Debug,Deserialize)]
pub(crate) struct ForexHelper{
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten)]
    forex: Option<HashMap<String, HashMap<String, EntryHelper>>>,
}

impl ForexHelper{
    pub(crate) fn convert(self) -> Forex{
        let mut forex = Forex::new();
        forex.error_message = self.error_message;
        forex.information = self.information;
        if let Some(meta_data) = self.meta_data{
            let information  = meta_data.get("1. Information").unwrap().clone();
            let from_symbol = meta_data.get("2. From Symbol").unwrap().clone();
            let to_symbol = meta_data.get("3. To Symbol").unwrap().clone();
            let last_refreshed  = meta_data.get("4. Last Refreshed");
            let mut last_refreshed_value = return_value(last_refreshed);
            if let None = last_refreshed_value{
                let last_refreshed  = meta_data.get("5. Last Refreshed");
                last_refreshed_value = return_value(last_refreshed);
            }
            let time_zone  = meta_data.get("5. Time Zone");
            let mut time_zone_value = return_value(time_zone);
            if let None = time_zone_value{
                let time_zone  = meta_data.get("6. Time Zone");
                time_zone_value = return_value(time_zone);
            }
            if let None = time_zone_value{
                let time_zone  = meta_data.get("7. Time Zone");
                time_zone_value = return_value(time_zone);
            }
            let output_size  = meta_data.get("4. Output Size");
            let mut output_size_value = return_value(output_size);
            if let None = output_size_value{
                let output_size  = meta_data.get("6. Output Size");
                output_size_value = return_value(output_size);
            }
            let interval = meta_data.get("5. Interval");
            let  interval_value = return_value(interval);
            forex.meta_data = Some(
            MetaData{
                information,
                from_symbol,
                to_symbol,
                last_refreshed : last_refreshed_value.unwrap(),
                interval : interval_value,
                output_size : output_size_value,
                time_zone : time_zone_value.unwrap(),
            }
            );
        }
        let mut value :Vec<Entry>= Vec::new();
        if let Some(entry) = self.forex{
            for hash in entry.values(){
                for val in hash.keys(){
                    let mut entry : Entry = Default::default();
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
        if !value.is_empty(){
            forex.forex = Some(value);
        }
        forex
    }
}

#[derive(Clone,Debug, Deserialize)]
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

fn return_value(value : Option<&std::string::String>) -> Option<String>{
    match value{
        Some(value) => Some(value.to_string()),
        None => None,
    }
}

pub fn create_url(
    function: ForexFunction,
    from_symbol: &str,
    to_symbol : &str,
    interval: Interval,
    output_size: OutputSize,
    api: String,
) -> Url {
    let function = match function {
        ForexFunction::IntraDay => "FX_INTRADAY",
        ForexFunction::Daily => "FX_DAILY",
        ForexFunction::Weekly => "FX_WEEKLY",
        ForexFunction::Monthly => "FX_MONTHLY",
    };

    let mut url = String::from(format!("{}{}&from_symbol={}&to_symbol={}", LINK, function, from_symbol,to_symbol));
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