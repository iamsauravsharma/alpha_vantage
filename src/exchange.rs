//! # Example
//! ```
//! fn exchnage_test() {
//!     let api = alpha_vantage::set_api("YOUR-API-HERE");
//!     assert_eq!(
//!         api.exchange("BTC", "CNY").name_from().unwrap(),
//!         String::from("Bitcoin")
//!     );
//! }
//! ```
//!
//! You can read about [Exchange][exchange] API and what it returns
//! on alphavantage documentation
//!
//! [exchange]: https://www.alphavantage.co/documentation/#currency-exchnage

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
    pub fn rate(&self) -> Result<f64, String> {
        if let Some(real) = self.real_time.clone() {
            Ok(real.rate.trim().parse::<f64>().unwrap())
        } else if let Some(error) = self.error_message.clone() {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    /// Get time when exchange rate was last refreshed.
    /// Example return value:- 2018-10-22 14:25:26 UTC.
    pub fn refreshed_time(&self) -> Result<String, String> {
        if let Some(real) = &self.real_time {
            Ok(format!("{} {}", real.last_refreshed, real.time_zone))
        } else if let Some(error) = self.error_message.clone() {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    /// get from code from which exchange is performed
    pub fn code_from(&self) -> Result<String, String> {
        self.get_result_string("from code")
    }

    /// get from name from which exchange is performed
    pub fn name_from(&self) -> Result<String, String> {
        self.get_result_string("from name")
    }

    /// get to code from exchange
    pub fn code_to(&self) -> Result<String, String> {
        self.get_result_string("to code")
    }

    /// get to name from exchange
    pub fn name_to(&self) -> Result<String, String> {
        self.get_result_string("to name")
    }

    /// Collect out certain value from real_time if presnt otherwise show error
    fn get_result_string(&self, match_str: &str) -> Result<String, String> {
        if let Some(real_time) = &self.real_time {
            let value = match match_str {
                "from code" => &real_time.from_code,
                "from name" => &real_time.from_name,
                "to code" => &real_time.to_code,
                "to name" => &real_time.to_name,
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
