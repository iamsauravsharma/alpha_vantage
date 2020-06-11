//! Module for exchange currency (both digital & physical currency exchange)
//!
//!  This API returns the realtime exchange rate for any pair of digital
//! currency (e.g., Bitcoin) or physical currency (e.g., USD).
//!
//! You can read about [Exchange][exchange] API and what it returns
//! on alphavantage documentation
//!
//! [exchange]: https://www.alphavantage.co/documentation/#currency-exchnage

use crate::error::{Error, Result};
use serde::Deserialize;

/// Struct used for helping creation of Exchange
#[derive(Debug, Deserialize)]
pub(crate) struct ExchangeHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Realtime Currency Exchange Rate")]
    real_time: Option<RealtimeExchangeRate>,
}

impl ExchangeHelper {
    pub(crate) fn convert(self) -> Result<Exchange> {
        let mut exchange = Exchange::default();
        if let Some(information) = self.information {
            return Err(Error::AlphaVantageInformation(information));
        }
        if let Some(error_message) = self.error_message {
            return Err(Error::AlphaVantageErrorMessage(error_message));
        }
        exchange.real_time = self.real_time.unwrap();
        Ok(exchange)
    }
}

/// Struct used for exchanging currency
#[derive(Default)]
pub struct Exchange {
    real_time: RealtimeExchangeRate,
}

/// Struct Storing Real time Exchange Value
#[derive(Debug, Deserialize, Clone, Default)]
struct RealtimeExchangeRate {
    #[serde(rename = "1. From_Currency Code")]
    from_code: String,
    #[serde(rename = "2. From_Currency Name")]
    from_name: String,
    #[serde(rename = "3. To_Currency Code")]
    to_code: String,
    #[serde(rename = "4. To_Currency Name")]
    to_name: String,
    #[serde(rename = "5. Exchange Rate")]
    rate: String,
    #[serde(rename = "6. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "7. Time Zone")]
    time_zone: String,
}

impl Exchange {
    /// Get Rate for exchange
    #[must_use]
    pub fn rate(&self) -> f64 {
        self.real_time
            .rate
            .trim()
            .parse::<f64>()
            .expect("failed to parse real_time rate to f64")
    }

    /// Get time when exchange rate was last refreshed along with time zone.
    #[must_use]
    pub fn refreshed_time(&self) -> &str {
        self.get_result_string("Refreshed time")
    }

    /// Return time zone of all data time
    #[must_use]
    pub fn time_zone(&self) -> &str {
        self.get_result_string("time zone")
    }

    /// get from code from which exchange is performed
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let exchange = api.exchange("BTC", "CNY").await.unwrap();
    ///     let code_from = exchange.code_from();
    ///     assert_eq!(code_from, "BTC");
    /// }
    /// ```
    #[must_use]
    pub fn code_from(&self) -> &str {
        self.get_result_string("from code")
    }

    /// get from name from which exchange is performed
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let exchange = api.exchange("BTC", "CNY").await.unwrap();
    ///     let name_from = exchange.name_from();
    ///     assert_eq!(name_from, "Bitcoin");
    /// }
    /// ```
    #[must_use]
    pub fn name_from(&self) -> &str {
        self.get_result_string("from name")
    }

    /// get to code from exchange
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let exchange = api.exchange("BTC", "CNY").await.unwrap();
    ///     let code_to = exchange.code_to();
    ///     assert_eq!(code_to, "CNY");
    /// }
    /// ```
    #[must_use]
    pub fn code_to(&self) -> &str {
        self.get_result_string("to code")
    }

    /// get to name from exchange
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let exchange = api.exchange("BTC", "CNY").await.unwrap();
    ///     let name_to = exchange.name_to();
    ///     assert_eq!(name_to, "Chinese Yuan");
    /// }
    /// ```
    #[must_use]
    pub fn name_to(&self) -> &str {
        self.get_result_string("to name")
    }

    /// Collect out certain value from real time
    fn get_result_string(&self, match_str: &str) -> &str {
        match match_str {
            "from code" => &self.real_time.from_code,
            "from name" => &self.real_time.from_name,
            "to code" => &self.real_time.to_code,
            "to name" => &self.real_time.to_name,
            "time zone" => &self.real_time.time_zone,
            "refreshed time" => &self.real_time.last_refreshed,
            _ => "",
        }
    }
}
