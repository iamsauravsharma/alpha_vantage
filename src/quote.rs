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


//Struct storing Global Quote Value
#[derive(Debug, Deserialize, Clone)]
struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    symbol: Option<String>,
    #[serde(rename = "02. open")]
    open: Option<String>,
    #[serde(rename = "03. high")]
    high: Option<String>,
    #[serde(rename = "04. low")]
    low: Option<String>,
    #[serde(rename = "05. price")]
    price: Option<String>,
    #[serde(rename = "06. volume")]
    volume: Option<String>,
    #[serde(rename = "07. latest trading day")]
    last_day: Option<String>,
    #[serde(rename = "08. previous close")]
    previous_close: Option<String>,
    #[serde(rename = "09. change")]
    change: Option<String>,
    #[serde(rename = "10. change percent")]
    change_percent: Option<String>,
}

impl Quote {
    /// return open value
    pub fn get_open(&self) -> Result<f64, String> {
        self.return_value("open")
    }

    /// return high value
    pub fn get_high(&self) -> Result<f64, String> {
        self.return_value("high")
    }

    /// return low value
    pub fn get_low(&self) -> Result<f64, String> {
        self.return_value("low")
    }

    /// return price value
    pub fn get_price(&self) -> Result<f64, String> {
        self.return_value("price")
    }

    /// return previous
    pub fn get_previous(&self) -> Result<f64, String> {
        self.return_value("previous")
    }

    ///return change
    pub fn get_change(&self) -> Result<f64, String> {
        self.return_value("change")
    }

    ///return change percent
    pub fn get_change_percent(&self) -> Result<f64, String> {
        let previous = self.get_previous()?;
        let price = self.get_price()?;
        Ok((price - previous) / previous)
    }

    // general function used for returning value of Quote method
    fn return_value(&self, value: &str) -> Result<f64, String> {
        if let Some(global) = self.global_quote.clone() {
            if let Some(price) = match value {
                "open" => global.open,
                "high" => global.high,
                "low" => global.low,
                "price" => global.price,
                "previous" => global.previous_close,
                "change" => global.change,
                _ => None,
            } {
                Ok(price.trim().parse::<f64>().unwrap())
            } else {
                Err("No value present please check Symbol again".to_string())
            }
        } else if let Some(error) = self.error_message.clone() {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    ///get last trading day
    pub fn get_last_trading(&self) -> Result<String, String> {
        if let Some(global) = self.global_quote.clone() {
            if let Some(value) = global.last_day {
                Ok(value)
            } else {
                Err("No value present please check Symbol again".to_string())
            }
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
