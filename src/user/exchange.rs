#[derive(Debug, Deserialize)]
pub struct Exchange {
    #[serde(rename = "Realtime Currency Exchange Rate")]
    real_time: RealtimeExchangeRate,
}

#[derive(Debug, Deserialize)]
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
    pub fn get_rate(&self) -> f64 {
        self.real_time.rate.clone().trim().parse::<f64>().unwrap()
    }
}
