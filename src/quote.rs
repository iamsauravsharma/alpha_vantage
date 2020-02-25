//! Module for returning latest price and volume information
//!
//! A lightweight alternative to the time series APIs, this service returns the
//! latest price and volume information for a security of your choice.
//!
//! You can read about [Quote][quote] API and what it returns
//! on alphavantage documentation
//!
//! [quote]: https://www.alphavantage.co/documentation/#latestprice

use crate::user::APIKey;
use serde::Deserialize;

/// Struct for helping creation of Quote
#[derive(Debug, Deserialize)]
pub(crate) struct QuoteHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Global Quote")]
    global_quote: Option<GlobalQuote>,
}

impl QuoteHelper {
    pub(crate) fn convert(self) -> Quote {
        let mut quote = Quote::default();
        quote.error_message = self.error_message;
        quote.information = self.information;
        quote.global_quote = self.global_quote;
        quote
    }
}

/// Struct for storing Quote related information
#[derive(Default)]
pub struct Quote {
    error_message: Option<String>,
    information: Option<String>,
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
    pub fn open(&self) -> Result<f64, &str> {
        self.return_f64_value("open")
    }

    /// return high value
    pub fn high(&self) -> Result<f64, &str> {
        self.return_f64_value("high")
    }

    /// return low value
    pub fn low(&self) -> Result<f64, &str> {
        self.return_f64_value("low")
    }

    /// return price value
    pub fn price(&self) -> Result<f64, &str> {
        self.return_f64_value("price")
    }

    /// return out a volume
    pub fn volume(&self) -> Result<f64, &str> {
        self.return_f64_value("volume")
    }

    /// return previous
    pub fn previous(&self) -> Result<f64, &str> {
        self.return_f64_value("previous")
    }

    /// return change
    pub fn change(&self) -> Result<f64, &str> {
        self.return_f64_value("change")
    }

    /// return change percent
    pub fn change_percent(&self) -> Result<f64, &str> {
        let previous = self.previous()?;
        let price = self.price()?;
        Ok((price - previous) / previous)
    }

    /// general function used for returning f64 value of Quote method
    fn return_f64_value(&self, value: &str) -> Result<f64, &str> {
        if let Some(global) = &self.global_quote {
            let price = match value {
                "open" => &global.open,
                "high" => &global.high,
                "low" => &global.low,
                "price" => &global.price,
                "previous" => &global.previous_close,
                "change" => &global.change,
                "volume" => &global.volume,
                _ => "",
            };
            Ok(price
                .trim()
                .parse::<f64>()
                .expect("failed to convert String to f64"))
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }

    /// get last trading day
    pub fn last_trading(&self) -> Result<&str, &str> {
        self.return_string_value("trading")
    }

    /// get symbol
    ///
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let quote = api.quote("MSFT");
    /// let symbol = quote.symbol();
    /// assert_eq!(symbol.unwrap(), "MSFT");
    /// ```
    pub fn symbol(&self) -> Result<&str, &str> {
        self.return_string_value("symbol")
    }

    /// general function used for returning String value
    fn return_string_value(&self, value: &str) -> Result<&str, &str> {
        if let Some(global) = &self.global_quote {
            let value = match value {
                "trading" => &global.last_day,
                "symbol" => &global.symbol,
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
}

/// Function used to create a [Quote][Quote] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
#[must_use]
pub fn quote(symbol: &str, api_data: (&str, Option<u64>)) -> Quote {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.quote(symbol)
}
