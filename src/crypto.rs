use crate::util::CryptoFunction;
use reqwest::Url;
use serde_derive::Deserialize;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

#[derive(Deserialize, Clone)]
struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Digital Currency Code")]
    digital_code: String,
    #[serde(rename = "3. Digital Currency Name")]
    digital_name: String,
    #[serde(rename = "4. Market Code")]
    market_code: String,
    #[serde(rename = "5. Market Name")]
    market_name: String,
    #[serde(rename = "6. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "7. Time Zone")]
    time_zone: String,
}

#[derive(Deserialize, Clone)]
struct EntryHelper {
    #[serde(rename = "1b. open (USD)")]
    open_usd: String,
    #[serde(rename = "2b. high (USD)")]
    high_usd: String,
    #[serde(rename = "3b. low (USD)")]
    low_usd: String,
    #[serde(rename = "4b. close (USD)")]
    close_usd: String,
    #[serde(rename = "5. volume")]
    volume: String,
    #[serde(rename = "6. market cap (USD)")]
    market_cap: String,
    #[serde(flatten)]
    market_data: HashMap<String, String>,
}

#[derive(Deserialize)]
pub(crate) struct CryptoHelper {
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<MetaData>,
    #[serde(flatten)]
    entry: Option<HashMap<String, HashMap<String, EntryHelper>>>,
}

impl CryptoHelper {
    pub(crate) fn convert(self) -> Crypto {
        let mut crypto = Crypto::default();
        crypto.information = self.information;
        crypto.error_message = self.error_message;
        crypto.meta_data = self.meta_data;
        if self.entry.is_some() {
            let mut vec_entry = Vec::new();
            for value in self.entry.unwrap().values() {
                for key in value.keys() {
                    let mut entry = Entry::default();
                    entry.time = key.to_string();
                    let entry_helper = value.get(key).unwrap().clone();
                    entry.usd_open = entry_helper.open_usd;
                    entry.usd_high = entry_helper.high_usd;
                    entry.usd_low = entry_helper.low_usd;
                    entry.usd_close = entry_helper.close_usd;
                    entry.market_cap = entry_helper.market_cap;
                    entry.volume = entry_helper.volume;
                    for key in entry_helper.market_data.keys() {
                        let value = &entry_helper.market_data[key];
                        let value = value.to_string();
                        if key.contains("1a") {
                            entry.market_open = value;
                        } else if key.contains("2a") {
                            entry.market_high = value;
                        } else if key.contains("3a") {
                            entry.market_low = value;
                        } else if key.contains("4a") {
                            entry.market_close = value;
                        }
                    }
                    vec_entry.push(entry);
                }
            }
            crypto.entry = Some(vec_entry);
        }
        crypto
    }
}

#[derive(Default, Debug, Clone)]
pub struct Entry {
    time: String,
    market_open: String,
    usd_open: String,
    market_high: String,
    usd_high: String,
    market_low: String,
    usd_low: String,
    market_close: String,
    usd_close: String,
    volume: String,
    market_cap: String,
}

impl Entry {
    pub fn time(&self) -> String {
        self.time.to_string()
    }

    pub fn market_open(&self) -> f64 {
        convert_to_f64(&self.market_open)
    }

    pub fn usd_open(&self) -> f64 {
        convert_to_f64(&self.usd_open)
    }

    pub fn market_high(&self) -> f64 {
        convert_to_f64(&self.market_high)
    }

    pub fn usd_high(&self) -> f64 {
        convert_to_f64(&self.usd_high)
    }

    pub fn market_low(&self) -> f64 {
        convert_to_f64(&self.market_low)
    }

    pub fn usd_low(&self) -> f64 {
        convert_to_f64(&self.usd_low)
    }

    pub fn market_close(&self) -> f64 {
        convert_to_f64(&self.market_close)
    }

    pub fn usd_close(&self) -> f64 {
        convert_to_f64(&self.usd_close)
    }

    pub fn volume(&self) -> f64 {
        convert_to_f64(&self.volume)
    }

    pub fn market_cap(&self) -> f64 {
        convert_to_f64(&self.market_cap)
    }
}

fn convert_to_f64(val: &str) -> f64 {
    val.trim().parse::<f64>().unwrap()
}

#[derive(Default)]
pub struct Crypto {
    information: Option<String>,
    error_message: Option<String>,
    meta_data: Option<MetaData>,
    entry: Option<Vec<Entry>>,
}

impl Crypto {
    pub fn information(&self) -> Result<String, String> {
        self.return_meta_string("information")
    }

    pub fn digital_code(&self) -> Result<String, String> {
        self.return_meta_string("digital code")
    }

    pub fn digital_name(&self) -> Result<String, String> {
        self.return_meta_string("digital name")
    }

    pub fn market_code(self) -> Result<String, String> {
        self.return_meta_string("market code")
    }

    pub fn market_name(&self) -> Result<String, String> {
        self.return_meta_string("market name")
    }

    pub fn last_refreshed(&self) -> Result<String, String> {
        self.return_meta_string("last refreshed")
    }

    pub fn time_zone(&self) -> Result<String, String> {
        self.return_meta_string("time zone")
    }

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

    fn return_meta_string(&self, which_val: &str) -> Result<String, String> {
        if let Some(meta_data) = &self.meta_data {
            let value = match which_val {
                "information" => &meta_data.information,
                "digital code" => &meta_data.digital_code,
                "digital name" => &meta_data.digital_name,
                "market code" => &meta_data.market_code,
                "market name" => &meta_data.market_name,
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
}

pub(crate) fn create_url(function: CryptoFunction, symbol: &str, market: &str, api: &str) -> Url {
    let function_name = match function {
        CryptoFunction::Daily => "DIGITAL_CURRENCY_DAILY",
        CryptoFunction::Weekly => "DIGITAL_CURRENCY_WEEKLY",
        CryptoFunction::Monthly => "DIGITAL_CURRENCY_MONTHLY",
    };
    let url = format!(
        "{}{}&symbol={}&market={}&apikey={}",
        LINK, function_name, symbol, market, api
    );
    url.parse().unwrap()
}
