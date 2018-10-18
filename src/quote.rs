#[derive(Debug, Deserialize)]
pub struct Quote {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Global Quote")]
    global_quote: Option<GlobalQuote>,
}

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
    pub fn get_open(&self) -> Result<f64, String> {
        self.return_value("open")
    }

    pub fn get_high(&self) -> Result<f64, String> {
        self.return_value("high")
    }

    pub fn get_low(&self) -> Result<f64, String> {
        self.return_value("low")
    }

    pub fn get_price(&self) -> Result<f64, String> {
        self.return_value("price")
    }

    pub fn get_previous(&self) -> Result<f64, String> {
        self.return_value("previous")
    }

    pub fn get_change(&self) -> Result<f64, String> {
        self.return_value("change")
    }

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
}
