//! Module for crypto real time data
//!
//! APIs under this section provide a wide range of data feed for digital and
//! crypto currencies such as Bitcoin.
//!
//! You can read about [Cryptocurrency][crypto_currency] API and what it returns
//! on alphavantage documentation
//!
//! [crypto_currency]: https://www.alphavantage.co/documentation/#digital-currency

use crate::{
    error::{Error, Result},
    user::APIKey,
    util::CryptoFunction,
};
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;

const LINK: &str = "https://www.alphavantage.co/query?function=";

/// Store Meta Data Information
#[derive(Deserialize, Clone, Default)]
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

/// Struct to help out for creation of struct Entry
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

/// Struct to help out for creation of struct Crypto
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
    /// Function which convert [CryptoHelper][CryptoHelper] to [Crypto][Crypto]
    pub(crate) fn convert(self) -> Result<Crypto> {
        let mut crypto = Crypto::default();
        if let Some(information) = self.information {
            return Err(Error::AlphaVantageInformation(information));
        }
        if let Some(error_message) = self.error_message {
            return Err(Error::AlphaVantageError(error_message));
        }
        crypto.meta_data = self.meta_data.unwrap();
        if self.entry.is_some() {
            let mut vec_entry = Vec::new();
            for value in self.entry.expect("self.entry is None").values() {
                for key in value.keys() {
                    let mut entry = Entry::default();
                    entry.time = key.to_string();
                    let entry_helper = value
                        .get(key)
                        .expect("failed to get out key from hashmap")
                        .clone();
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
            crypto.entry = vec_entry;
        }
        Ok(crypto)
    }
}

/// Struct which stores Crypto data
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
                    .expect(&format!("Failed to find time value for index {}", i));
                full_list.push(entry);
            } else {
                return Err(Error::DesiredNumberOfEntryNotPresent(time_list_count));
            }
        }
        Ok(full_list)
    }
}

impl Entry {
    /// Return time
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return market open value
    #[must_use]
    pub fn market_open(&self) -> f64 {
        convert_to_f64(&self.market_open)
    }

    /// Return usd open value
    #[must_use]
    pub fn usd_open(&self) -> f64 {
        convert_to_f64(&self.usd_open)
    }

    /// Return market high value
    #[must_use]
    pub fn market_high(&self) -> f64 {
        convert_to_f64(&self.market_high)
    }

    /// Return usd high value
    #[must_use]
    pub fn usd_high(&self) -> f64 {
        convert_to_f64(&self.usd_high)
    }

    /// Return market low value
    #[must_use]
    pub fn market_low(&self) -> f64 {
        convert_to_f64(&self.market_low)
    }

    /// Return usd low value
    #[must_use]
    pub fn usd_low(&self) -> f64 {
        convert_to_f64(&self.usd_low)
    }

    /// Return market close value
    #[must_use]
    pub fn market_close(&self) -> f64 {
        convert_to_f64(&self.market_close)
    }

    /// Return usd close value
    #[must_use]
    pub fn usd_close(&self) -> f64 {
        convert_to_f64(&self.usd_close)
    }

    /// Return volume
    #[must_use]
    pub fn volume(&self) -> f64 {
        convert_to_f64(&self.volume)
    }

    /// Return market cap
    #[must_use]
    pub fn market_cap(&self) -> f64 {
        convert_to_f64(&self.market_cap)
    }
}

/// Convert String to f64
fn convert_to_f64(val: &str) -> f64 {
    val.trim()
        .parse::<f64>()
        .expect("Failed to convert String to f64")
}

/// Struct which holds out Crypto currency information
#[derive(Default)]
pub struct Crypto {
    meta_data: MetaData,
    entry: Vec<Entry>,
}

impl Crypto {
    /// Return meta data information
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto = api
    ///         .crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY")
    ///         .await
    ///         .unwrap();
    ///     let information = crypto.information();
    ///     assert_eq!(information, "Daily Prices and Volumes for Digital Currency");
    /// }
    /// ```
    #[must_use]
    pub fn information(&self) -> &str {
        self.return_meta_string("information")
    }

    /// Return digital currency code
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto = api
    ///         .crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY")
    ///         .await
    ///         .unwrap();
    ///     let digital_code = crypto.digital_code();
    ///     assert_eq!(digital_code, "BTC");
    /// }
    /// ```
    #[must_use]
    pub fn digital_code(&self) -> &str {
        self.return_meta_string("digital code")
    }

    /// Return digital currency name
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto = api
    ///         .crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY")
    ///         .await
    ///         .unwrap();
    ///     let digital_name = crypto.digital_name();
    ///     assert_eq!(digital_name, "Bitcoin");
    /// }
    /// ```
    #[must_use]
    pub fn digital_name(&self) -> &str {
        self.return_meta_string("digital name")
    }

    /// Return market code
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto = api
    ///         .crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY")
    ///         .await
    ///         .unwrap();
    ///     let market_code = crypto.market_code();
    ///     assert_eq!(market_code, "CNY");
    /// }
    /// ```
    #[must_use]
    pub fn market_code(&self) -> &str {
        self.return_meta_string("market code")
    }

    /// Return market name
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto = api
    ///         .crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY")
    ///         .await
    ///         .unwrap();
    ///     let market_name = crypto.market_name();
    ///     assert_eq!(market_name, "Chinese Yuan");
    /// }
    /// ```
    #[must_use]
    pub fn market_name(&self) -> &str {
        self.return_meta_string("market name")
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

    /// Return out a entry
    #[must_use]
    pub fn entry(&self) -> &Vec<Entry> {
        &self.entry
    }

    /// Return meta string
    fn return_meta_string(&self, which_val: &str) -> &str {
        match which_val {
            "information" => &self.meta_data.information,
            "digital code" => &self.meta_data.digital_code,
            "digital name" => &self.meta_data.digital_name,
            "market code" => &self.meta_data.market_code,
            "market name" => &self.meta_data.market_name,
            "time zone" => &self.meta_data.time_zone,
            "last refreshed" => &self.meta_data.last_refreshed,
            _ => "",
        }
    }
}

/// Function used to create a [Crypto][Crypto] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub async fn crypto(
    function: CryptoFunction,
    symbol: &str,
    market: &str,
    api_data: (&str, Option<u64>),
) -> Result<Crypto> {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.crypto(function, symbol, market).await
}

/// Create url from which JSON data is collected for Crypto
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
    url.parse().expect("Failed to parse an url")
}

#[cfg(test)]
mod test {
    use crate::util::*;
    use reqwest::Url;
    #[test]
    fn test_crypto_create_url() {
        assert_eq!(
            super::create_url(CryptoFunction::Daily, "BTC", "USD", "random"),
            Url::parse(
                "https://www.alphavantage.co/query?function=DIGITAL_CURRENCY_DAILY\
                 &symbol=BTC\
                 &market=USD\
                 &apikey=random"
            )
            .unwrap()
        );
        assert_eq!(
            super::create_url(CryptoFunction::Weekly, "ETH", "EUR", "randomkey"),
            Url::parse(
                "https://www.alphavantage.co/query?function=DIGITAL_CURRENCY_WEEKLY\
                 &symbol=ETH\
                 &market=EUR\
                 &apikey=randomkey"
            )
            .unwrap()
        );
        assert_eq!(
            super::create_url(CryptoFunction::Monthly, "BTC", "CNY", "demo"),
            Url::parse(
                "https://www.alphavantage.co/query?function=DIGITAL_CURRENCY_MONTHLY\
                 &symbol=BTC\
                 &market=CNY\
                 &apikey=demo"
            )
            .unwrap()
        );
    }
}
