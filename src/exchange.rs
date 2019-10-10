//! Module for exchange currency (both digital & physical currency exchange)
//!
//!  This API returns the realtime exchange rate for any pair of digital
//! currency (e.g., Bitcoin) or physical currency (e.g., USD).
//!
//! You can read about [Exchange][exchange] API and what it returns
//! on alphavantage documentation
//!
//! [exchange]: https://www.alphavantage.co/documentation/#currency-exchnage

use crate::user::APIKey;
use serde_derive::Deserialize;

/// Struct used for exchanging currency
#[derive(Debug, Deserialize)]
pub struct Exchange {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Realtime Currency Exchange Rate")]
    real_time: Option<RealtimeExchangeRate>,
}

/// Struct Storing Real time Exchange Value
#[derive(Debug, Deserialize, Clone)]
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
    /// Get Rate for exchange produce error if no rate is available
    pub fn rate(&self) -> Result<f64, &str> {
        if let Some(real) = &self.real_time {
            Ok(real
                .rate
                .trim()
                .parse::<f64>()
                .expect("failed to parse real_time rate to f64"))
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }

    /// Get time when exchange rate was last refreshed along with time zone.
    pub fn refreshed_time(&self) -> Result<&str, &str> {
        self.get_result_string("Refreshed time")
    }

    /// Return time zone of all data time produce error if API return
    /// error message or information instead of meta data
    pub fn time_zone(&self) -> Result<&str, &str> {
        self.get_result_string("time zone")
    }

    /// get from code from which exchange is performed
    ///
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let exchange = api.exchange("BTC", "CNY");
    /// let code_from = exchange.code_from();
    /// assert_eq!(code_from.unwrap(), "BTC");
    /// ```
    pub fn code_from(&self) -> Result<&str, &str> {
        self.get_result_string("from code")
    }

    /// get from name from which exchange is performed
    ///
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let exchange = api.exchange("BTC", "CNY");
    /// let name_from = exchange.name_from();
    /// assert_eq!(name_from.unwrap(), "Bitcoin");
    /// ```
    pub fn name_from(&self) -> Result<&str, &str> {
        self.get_result_string("from name")
    }

    /// get to code from exchange
    ///
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let exchange = api.exchange("BTC", "CNY");
    /// let code_to = exchange.code_to();
    /// assert_eq!(code_to.unwrap(), "CNY");
    /// ```
    pub fn code_to(&self) -> Result<&str, &str> {
        self.get_result_string("to code")
    }

    /// get to name from exchange
    ///
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let exchange = api.exchange("BTC", "CNY");
    /// let name_to = exchange.name_to();
    /// assert_eq!(name_to.unwrap(), "Chinese Yuan");
    /// ```
    pub fn name_to(&self) -> Result<&str, &str> {
        self.get_result_string("to name")
    }

    /// Collect out certain value from real time if present otherwise show error
    fn get_result_string(&self, match_str: &str) -> Result<&str, &str> {
        if let Some(real_time) = &self.real_time {
            let value = match match_str {
                "from code" => &real_time.from_code,
                "from name" => &real_time.from_name,
                "to code" => &real_time.to_code,
                "to name" => &real_time.to_name,
                "time zone" => &real_time.time_zone,
                "refreshed time" => &real_time.last_refreshed,
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

/// Function used to create a [Exchange][Exchange] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub fn exchange(from: &str, to: &str, api_data: (&str, Option<u64>)) -> Exchange {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.exchange(from, to)
}
