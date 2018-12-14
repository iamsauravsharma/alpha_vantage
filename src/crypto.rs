use crate::util::CryptoFunction;
use reqwest::Url;
use serde_derive::Deserialize;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

#[derive(Default)]
pub struct Crypto {
    information: Option<String>,
    error_message: Option<String>,
    meta_data: Option<MetaData>,
    entry: Option<Vec<Entry>>,
}

impl Crypto {
    pub fn meta_data(&self) -> &Option<MetaData> {
        &self.meta_data
    }

    pub fn entry(&self) -> &Option<Vec<Entry>> {
        &self.entry
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
    pub fn time(self) -> String {
        self.time.to_string()
    }

    pub fn market_open(self) -> String {
        self.market_open.to_string()
    }

    pub fn usd_open(self) -> String {
        self.usd_open.to_string()
    }

    pub fn market_high(self) -> String {
        self.market_high.to_string()
    }

    pub fn usd_high(self) -> String {
        self.usd_high.to_string()
    }

    pub fn market_low(self) -> String {
        self.market_low.to_string()
    }

    pub fn usd_low(self) -> String {
        self.usd_low.to_string()
    }

    pub fn market_close(self) -> String {
        self.market_close.to_string()
    }

    pub fn usd_close(self) -> String {
        self.usd_close.to_string()
    }

    pub fn volume(self) -> String {
        self.volume.to_string()
    }

    pub fn market_cap(self) -> String {
        self.market_cap.to_string()
    }
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

#[derive(Deserialize, Clone)]
pub struct MetaData {
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

impl MetaData {
    pub fn information(&self) -> String {
        self.information.to_string()
    }

    pub fn digital_code(&self) -> String {
        self.digital_code.to_string()
    }

    pub fn digital_name(&self) -> String {
        self.digital_name.to_string()
    }

    pub fn market_code(self) -> String {
        self.market_code.to_string()
    }

    pub fn market_name(&self) -> String {
        self.market_name.to_string()
    }

    pub fn last_refreshed(&self) -> String {
        self.last_refreshed.to_string()
    }

    pub fn time_zone(&self) -> String {
        self.time_zone.to_string()
    }
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
