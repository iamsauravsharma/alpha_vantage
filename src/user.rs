pub struct APIKey(String);

use crate::exchange::Exchange;
use crate::quote::Quote;
use crate::time_series::*;
use crate::util::*;
use crate::search::*;
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

    pub fn quote(&self, symbol: &str) -> Quote {
        let data: Url = format!(
            "{}GLOBAL_QUOTE&symbol={}&apikey={}",
            LINK,
            symbol,
            self.0.clone()
        )
        .parse()
        .unwrap();

        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }

    pub fn stock_time(
        &self,
        function: Function,
        symbol: &str,
        interval: Interval,
        output_size: OutputSize,
    ) -> TimeSeries {
        let data: Url = create_url(function, symbol, interval, output_size, self.0.clone());
        let time_series_helper  : TimeSeriesHelper = serde_json::from_str(&get(data).unwrap().text().unwrap()).unwrap();
        time_series_helper.convert()
    }

    pub fn search(&self,keywords : &str) -> Search{
        let data :Url  = format!("{}SYMBOL_SEARCH&keywords={}&apikey={}",LINK,keywords,self.0.clone()).parse().unwrap();
        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }
}
