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

//Struct Storing Real time Exchange Value
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
    /// Get Rate for exchange
    pub fn get_rate(&self) -> Result<f64, String> {
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

    #[doc = "Get time when exchange rate was last refreshed. \n"]
    #[doc = "Example return value:- 2018-10-22 14:25:26 UTC."]
    pub fn get_refreshed_time(&self) -> Result<String, String> {
        if let Some(real) = self.real_time.clone() {
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
}
