//! # Note
//! A lightweight alternative to the time series APIs, this service returns the
//! latest price and volume information for a security of your choice.
//!
//! # Example
//! ```
//! fn quote_function() {
//!     let api = alpha_vantage::set_api("YOUR-API-HERE");
//!     let quote = api.quote("MSFT");
//!     assert_eq!(quote.open().is_ok(), true);
//! }
//! ```
//!
//! You can read about [Quote][quote] API and what it returns
//! on alphavantage documentation
//!
//! [quote]: https://www.alphavantage.co/documentation/#latestprice

use crate::user::APIKey;
use serde_derive::Deserialize;

/// Struct for storing Quote related information
#[derive(Debug, Deserialize)]
pub struct Quote {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Global Quote")]
    global_quote: Option<GlobalQuote>,
}

/// Struct storing Global Quote Value
#[derive(Debug, Deserialize, Clone)]
struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(rename = "02. open")]
    open: String,
    #[serde(rename = "03. high")]
    high: String,
    #[serde(rename = "04. low")]
    low: String,
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "06. volume")]
    volume: String,
    #[serde(rename = "07. latest trading day")]
    last_day: String,
    #[serde(rename = "08. previous close")]
    previous_close: String,
    #[serde(rename = "09. change")]
    change: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

impl Quote {
    /// return open value
    pub fn open(&self) -> Result<f64, String> {
        self.return_f64_value("open")
    }

    /// return high value
    pub fn high(&self) -> Result<f64, String> {
        self.return_f64_value("high")
    }

    /// return low value
    pub fn low(&self) -> Result<f64, String> {
        self.return_f64_value("low")
    }

    /// return price value
    pub fn price(&self) -> Result<f64, String> {
        self.return_f64_value("price")
    }

    /// return previous
    pub fn previous(&self) -> Result<f64, String> {
        self.return_f64_value("previous")
    }

    /// return change
    pub fn change(&self) -> Result<f64, String> {
        self.return_f64_value("change")
    }

    /// return change percent
    pub fn change_percent(&self) -> Result<f64, String> {
        let previous = self.previous()?;
        let price = self.price()?;
        Ok((price - previous) / previous)
    }

    /// general function used for returning f64 value of Quote method
    fn return_f64_value(&self, value: &str) -> Result<f64, String> {
        if let Some(global) = self.global_quote.clone() {
            let price = match value {
                "open" => global.open,
                "high" => global.high,
                "low" => global.low,
                "price" => global.price,
                "previous" => global.previous_close,
                "change" => global.change,
                _ => "".to_string(),
            };
            return Ok(price.trim().parse::<f64>().unwrap());
        } else if let Some(error) = self.error_message.clone() {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    /// get last trading day
    pub fn last_trading(&self) -> Result<String, String> {
        self.return_string_value("trading")
    }

    /// get symbol
    pub fn symbol(&self) -> Result<String, String> {
        self.return_string_value("symbol")
    }

    /// general function used for returning String value
    fn return_string_value(&self, value: &str) -> Result<String, String> {
        if let Some(global) = self.global_quote.clone() {
            let value = match value {
                "trading" => global.last_day,
                "symbol" => global.symbol,
                _ => "".to_string(),
            };
            return Ok(value);
        } else if let Some(error) = self.error_message.clone() {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }
}

/// Function used to create a [Quote][Quote] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub fn quote(symbol: &str, api_data: (&str, Option<u64>)) -> Quote {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.quote(symbol)
}
