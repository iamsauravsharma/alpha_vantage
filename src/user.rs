use crate::{
    crypto::{create_url as create_url_crypto, Crypto, CryptoHelper},
    exchange::Exchange,
    forex::{create_url as create_url_forex, Forex, ForexHelper},
    quote::Quote,
    search::*,
    sector::{Sector, SectorHelper},
    stock_time::{create_url as create_url_time_series, TimeSeries, TimeSeriesHelper},
    technical_indicator::{create_url as create_url_technical, Indicator},
    util::*,
};
use reqwest::{get, Url};

const LINK: &str = "https://www.alphavantage.co/query?function=";

/// Struct for initializing apikey value as well as contain different method for
/// API call
pub struct APIKey(String);

impl APIKey {
    /// Method for initializing APIKey struct
    pub fn set_api(api: &str) -> Self {
        APIKey(api.to_string())
    }

    /// Method to get api key
    pub fn get_api(&self) -> String {
        self.0.clone()
    }

    /// Crypto method for calling cryptography function
    ///
    /// # Example
    /// ```
    /// fn crypto_function() {
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     let crypto = api.crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "USD");
    ///     let digital_name = crypto.digital_name();
    ///     assert_eq!(digital_name.unwrap(), String::from("Bitcoin"));
    /// }
    /// ```
    pub fn crypto(&self, function: CryptoFunction, symbol: &str, market: &str) -> Crypto {
        let data: Url = create_url_crypto(function, symbol, market, &self.0);
        let crypto_helper: CryptoHelper =
            serde_json::from_str(&get(data).unwrap().text().unwrap()).unwrap();
        crypto_helper.convert()
    }

    /// Method for exchanging currency value from one currency to another
    /// currency.
    ///
    /// # Example
    /// ```
    /// fn exchange_function() {
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     assert_eq!(
    ///         api.exchange("BTC", "CNY").name_from().unwrap(),
    ///         String::from("Bitcoin")
    ///     );
    /// }
    /// ```
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

    /// Forex method for calling stock time series
    ///
    /// # Example
    /// ```
    /// fn forex_function() {
    ///     use alpha_vantage::util::*;
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     let forex = api.forex(
    ///         ForexFunction::Weekly,
    ///         "MSFT",
    ///         "BTC",
    ///         Interval::None,
    ///         OutputSize::None,
    ///     );
    ///     assert_eq!(forex.symbol_from().unwrap(), "MSFT".to_string());
    /// }
    /// ```
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

    /// Method for returning Quote Struct
    /// # Example
    /// ```
    /// fn quote_function() {
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     let quote = api.quote("MSFT");
    ///     assert_eq!(quote.open().is_ok(), true);
    /// }
    /// ```
    pub fn quote(&self, symbol: &str) -> Quote {
        let data: Url = format!("{}GLOBAL_QUOTE&symbol={}&apikey={}", LINK, symbol, self.0)
            .parse()
            .unwrap();

        let body = get(data).unwrap().text().unwrap();
        serde_json::from_str(&body).unwrap()
    }

    /// Search method for searching keyword or company
    /// # Example
    /// ```
    /// fn search_function() {
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     let search = api.search("BA");
    ///     assert_eq!(search.result().is_ok(), true);
    /// }
    /// ```
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

    /// Method for returning out a sector data as struct
    /// # Example
    /// ```
    /// fn sector_function() {
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     let sector = api.sector();
    ///     assert_eq!(sector.information().is_ok(), true);
    /// }
    /// ```
    pub fn sector(&self) -> Sector {
        let data: Url = format!("{}SECTOR&apikey={}", LINK, self.0).parse().unwrap();
        let body = get(data).unwrap().text().unwrap();
        let sector_helper: SectorHelper = serde_json::from_str(&body).unwrap();
        sector_helper.convert()
    }

    /// Stock time method for calling stock time series API
    /// # Example
    /// ```
    /// fn time_series_function() {
    ///     use alpha_vantage::util::*;
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     let stock = api.stock_time(
    ///         StockFunction::Weekly,
    ///         "MSFT",
    ///         Interval::None,
    ///         OutputSize::None,
    ///     );
    ///     assert_eq!(stock.symbol().unwrap(), "MSFT".to_string());
    /// }
    /// ```
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

    /// Technical indicator API caller method
    /// # Example
    /// ```
    /// fn technical_indicator_function() {
    ///     let api = alpha_vantage::set_api("YOUR-API-HERE");
    ///     let technical =
    ///         api.technical_indicator("SEMA", "MSFT", "1min", Some("open"), Some("10"), vec![]);
    ///     assert_eq!(technical.data().is_ok(), true);
    /// }
    /// ```
    pub fn technical_indicator(
        &self,
        function: &str,
        symbol: &str,
        interval: &str,
        series_type: Option<&str>,
        time_period: Option<&str>,
        temporary_value: Vec<TechnicalIndicator>,
    ) -> Indicator {
        let url = create_url_technical(
            function,
            symbol,
            interval,
            series_type,
            time_period,
            temporary_value,
            &self.0,
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
