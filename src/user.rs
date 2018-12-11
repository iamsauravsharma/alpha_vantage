/// Struct for initializing apikey value
pub struct APIKey(String);

use crate::{
    crypto::{create_url as create_url_crypto, Crypto, CryptoHelper},
    exchange::Exchange,
    forex::{create_url as create_url_forex, Forex, ForexHelper},
    quote::Quote,
    search::*,
    sector::{Sector, SectorHelper},
    technical_indicator::{create_url as create_url_technical, Indicator},
    time_series::{create_url as create_url_time_series, TimeSeries, TimeSeriesHelper},
    util::*,
};
use reqwest::{get, Url};

const LINK: &str = "https://www.alphavantage.co/query?function=";

impl APIKey {
    /// Method for initializing APIKey struct
    pub fn set_api(api: &str) -> Self {
        APIKey(api.to_string())
    }

    /// Method to get api key
    pub fn get_api(&self) -> String {
        self.0.clone()
    }

    /// Method for exchanging currency value from one currency to another
    /// currency.
    pub fn exchange(&self, from_currency: &str, to_currency: &str) -> Exchange {
        let data: Url = format!(
            "{}CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}",
            LINK, from_currency, to_currency, self.0
        )
        .parse()
        .unwrap();

        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }

    /// Method for returning Quote Struct
    pub fn quote(&self, symbol: &str) -> Quote {
        let data: Url = format!("{}GLOBAL_QUOTE&symbol={}&apikey={}", LINK, symbol, self.0)
            .parse()
            .unwrap();

        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }

    /// Stock time method for calling stock time series API
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

    /// Search method for searching keyword or company
    pub fn search(&self, keywords: &str) -> Search {
        let data: Url = format!(
            "{}SYMBOL_SEARCH&keywords={}&apikey={}",
            LINK, keywords, self.0
        )
        .parse()
        .unwrap();
        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }

    pub fn sector(&self) -> Sector {
        let data: Url = format!("{}SECTOR&apikey={}", LINK, self.0).parse().unwrap();
        let body = get(data).unwrap().text().unwrap();
        let sector_helper: SectorHelper = serde_json::from_str(&body).unwrap();
        sector_helper.convert()
    }

    /// Forex method for calling stock time series
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

    pub fn crypto(&self, function: CryptoFunction, symbol: &str, market: &str) -> Crypto {
        let data: Url = create_url_crypto(function, symbol, market, &self.0);
        let crypto_helper: CryptoHelper =
            serde_json::from_str(&get(data).unwrap().text().unwrap()).unwrap();
        crypto_helper.convert()
    }

    pub fn technical_indicator(
        function: &str,
        symbol: &str,
        interval: &str,
        apikey: &str,
        series_type: Option<&str>,
        time_period: Option<&str>,
        temporary_value: Vec<TechnicalIndicator>,
    ) -> Indicator {
        let url = create_url_technical(
            function,
            symbol,
            interval,
            apikey,
            series_type,
            time_period,
            temporary_value,
        );
        serde_json::from_str(&get(url).unwrap().text().unwrap()).unwrap()
    }
}

// Mod for unit testing
#[cfg(test)]
mod test {
    #[test]
    // Testing get api and set api function
    fn test_get_api() {
        assert_eq!(super::APIKey::set_api("demo").get_api(), "demo".to_string());
    }
}
