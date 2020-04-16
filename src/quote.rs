//! Module for returning latest price and volume information
//!
//! A lightweight alternative to the time series APIs, this service returns the
//! latest price and volume information for a security of your choice.
//!
//! You can read about [Quote][quote] API and what it returns
//! on alphavantage documentation
//!
//! [quote]: https://www.alphavantage.co/documentation/#latestprice

use crate::{
    error::{Error, Result},
    user::APIKey,
};
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
    pub(crate) fn convert(self) -> Result<Quote> {
        let mut quote = Quote::default();
        if let Some(information) = self.information {
            return Err(Error::AlphaVantageInformation(information));
        }
        if let Some(error_message) = self.error_message {
            return Err(Error::AlphaVantageError(error_message));
        }
        quote.global_quote = self.global_quote.unwrap();
        Ok(quote)
    }
}

/// Struct for storing Quote related information
#[derive(Default)]
pub struct Quote {
    global_quote: GlobalQuote,
}

/// Struct storing Global Quote Value
#[derive(Debug, Deserialize, Clone, Default)]
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
    #[must_use]
    pub fn open(&self) -> f64 {
        self.return_f64_value("open")
    }

    /// return high value
    #[must_use]
    pub fn high(&self) -> f64 {
        self.return_f64_value("high")
    }

    /// return low value
    #[must_use]
    pub fn low(&self) -> f64 {
        self.return_f64_value("low")
    }

    /// return price value
    #[must_use]
    pub fn price(&self) -> f64 {
        self.return_f64_value("price")
    }

    /// return out a volume
    #[must_use]
    pub fn volume(&self) -> f64 {
        self.return_f64_value("volume")
    }

    /// return previous
    #[must_use]
    pub fn previous(&self) -> f64 {
        self.return_f64_value("previous")
    }

    /// return change
    #[must_use]
    pub fn change(&self) -> f64 {
        self.return_f64_value("change")
    }

    /// return change percent
    #[must_use]
    pub fn change_percent(&self) -> f64 {
        let previous = self.previous();
        let price = self.price();
        (price - previous) / previous
    }

    /// general function used for returning f64 value of Quote method
    fn return_f64_value(&self, value: &str) -> f64 {
        let price = match value {
            "open" => &self.global_quote.open,
            "high" => &self.global_quote.high,
            "low" => &self.global_quote.low,
            "price" => &self.global_quote.price,
            "previous" => &self.global_quote.previous_close,
            "change" => &self.global_quote.change,
            "volume" => &self.global_quote.volume,
            _ => "",
        };
        price
            .trim()
            .parse::<f64>()
            .expect("failed to convert String to f64")
    }

    /// get last trading day
    #[must_use]
    pub fn last_trading(&self) -> &str {
        self.return_string_value("trading")
    }

    /// get symbol
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let quote = api.quote("MSFT").await.unwrap();
    ///     let symbol = quote.symbol();
    ///     assert_eq!(symbol, "MSFT");
    /// }
    /// ```
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.return_string_value("symbol")
    }

    /// general function used for returning String value
    fn return_string_value(&self, value: &str) -> &str {
        match value {
            "trading" => &self.global_quote.last_day,
            "symbol" => &self.global_quote.symbol,
            _ => "",
        }
    }
}

/// Function used to create a [Quote][Quote] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub async fn quote(symbol: &str, api_data: (&str, Option<u64>)) -> Result<Quote> {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.quote(symbol).await
}
