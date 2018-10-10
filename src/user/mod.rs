pub struct APIKey(String);
pub mod exchange;

use self::exchange::Exchange;
use reqwest::{get, Url};

const LINK: &str = "https://www.alphavantage.co/query?function=";

impl APIKey {
    pub fn set_api(api: &str) -> APIKey {
        APIKey(api.to_string())
    }

    pub fn get_api(&self) -> String {
        self.0.clone()
    }

    pub fn exchange(&self, from_currency: &str, to_currency: &str) -> Exchange {
        let data: Url = format!(
            "{}CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}",
            LINK,
            from_currency,
            to_currency,
            self.0.clone()
        )
        .parse()
        .unwrap();

        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }
}
