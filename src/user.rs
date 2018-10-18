pub struct APIKey(String);

use crate::exchange::Exchange;
use crate::forex::create_url as create_url_forex;
use crate::forex::{Forex, ForexHelper};
use crate::quote::Quote;
use crate::search::*;
use crate::time_series::create_url as create_url_time_series;
use crate::time_series::{TimeSeries, TimeSeriesHelper};
use crate::util::*;
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
        function: StockFunction,
        symbol: &str,
        interval: Interval,
        output_size: OutputSize,
    ) -> TimeSeries {
        let data: Url = create_url_time_series(function, symbol, interval, output_size, &self.0);
        let time_series_helper: TimeSeriesHelper =
            serde_json::from_str(&get(data).unwrap().text().unwrap()).unwrap();
        time_series_helper.convert()
    }

    pub fn search(&self, keywords: &str) -> Search {
        let data: Url = format!(
            "{}SYMBOL_SEARCH&keywords={}&apikey={}",
            LINK,
            keywords,
            self.0.clone()
        )
        .parse()
        .unwrap();
        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }

    pub fn forex(
        &self,
        function: ForexFunction,
        from_symbol: &str,
        to_symbol: &str,
        interval: Interval,
        output_size: OutputSize,
    ) -> Forex {
        let data: Url = create_url_forex(
            function,
            from_symbol,
            to_symbol,
            interval,
            output_size,
            &self.0,
        );
        let forex_helper: ForexHelper =
            serde_json::from_str(&get(data).unwrap().text().unwrap()).unwrap();
        forex_helper.convert()
    }
}
