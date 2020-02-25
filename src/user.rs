use crate::{
    crypto::{create_url as create_url_crypto, Crypto, CryptoHelper},
    exchange::{Exchange, ExchangeHelper},
    forex::{create_url as create_url_forex, Forex, ForexHelper},
    quote::{Quote, QuoteHelper},
    search::{Search, SearchHelper},
    sector::{Sector, SectorHelper},
    stock_time::{create_url as create_url_time_series, TimeSeries, TimeSeriesHelper},
    technical_indicator::{create_url as create_url_technical, Indicator, IndicatorHelper},
    util::{
        CryptoFunction, ForexFunction, Interval, OutputSize, StockFunction, TechnicalIndicator,
    },
};
use reqwest::{
    blocking::{Client, ClientBuilder},
    Url,
};

const LINK: &str = "https://www.alphavantage.co/query?function=";

/// Struct for initializing api key value as well as contain different method
/// for API call
pub struct APIKey {
    api: String,
    timeout: u64,
    client: Client,
}

impl APIKey {
    /// Method for initializing [APIKey][APIKey] struct
    ///
    /// ```
    /// use alpha_vantage::user::APIKey;
    /// let api = APIKey::set_api("some_key");
    /// ```
    #[must_use]
    pub fn set_api(api: &str) -> Self {
        let client = ClientBuilder::new()
            .build()
            .expect("Failed to build out Client Builder");
        Self {
            api: api.to_string(),
            timeout: 30,
            client,
        }
    }

    /// Set API value with timeout period
    ///
    /// ```
    /// use alpha_vantage::user::APIKey;
    /// let api_with_custom_timeout = APIKey::set_with_timeout("your_api_key", 45);
    /// ```
    #[must_use]
    pub fn set_with_timeout(api: &str, timeout: u64) -> Self {
        let client = ClientBuilder::new()
            .timeout(Some(std::time::Duration::from_secs(timeout)))
            .build()
            .expect("Failed to build out Client Builder with timeout");
        Self {
            api: api.to_string(),
            timeout,
            client,
        }
    }

    /// Set out [APIKey][APIKey] by reading out environment variable
    ///
    /// ```
    /// use alpha_vantage::user::APIKey;
    /// std::env::set_var("KEY_NAME", "some_key");
    /// let api_from_env = APIKey::set_with_env("KEY_NAME");
    /// assert_eq!(api_from_env.get_api(), "some_key");
    /// ```
    #[must_use]
    pub fn set_with_env(env_name: &str) -> Self {
        let api = std::env::var(env_name).expect("environment variable is not present");
        let client = ClientBuilder::new()
            .build()
            .expect("Failed to build out Client Builder");
        Self {
            api,
            timeout: 30,
            client,
        }
    }

    /// Update timeout for API key
    ///
    /// ```
    /// use alpha_vantage::user::APIKey;
    /// let mut api = alpha_vantage::user::APIKey::set_api("some_key");
    /// assert_eq!(api.get_timeout(), 30_u64);
    /// api.update_timeout(60_u64);
    /// assert_eq!(api.get_timeout(), 60_u64);
    /// ```
    pub fn update_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    /// Method to get api key
    ///
    /// ```
    /// use alpha_vantage::user::APIKey;
    /// let api = alpha_vantage::user::APIKey::set_api("some_key");
    /// assert_eq!(api.get_api(), "some_key");
    /// ```
    #[must_use]
    pub fn get_api(&self) -> &str {
        &self.api
    }

    /// Get API timeout period
    ///
    /// ```
    /// use alpha_vantage::user::APIKey;
    /// let api_with_custom_timeout = APIKey::set_with_timeout("your_api_key", 45);
    /// assert_eq!(api_with_custom_timeout.get_timeout(), 45_u64);
    /// ```
    #[must_use]
    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }

    /// Crypto method for calling cryptography function
    ///
    /// # Example
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let crypto = api.crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY");
    /// let digital_name = crypto.digital_name();
    /// assert_eq!(digital_name.unwrap(), String::from("Bitcoin"));
    /// ```
    #[must_use]
    pub fn crypto(&self, function: CryptoFunction, symbol: &str, market: &str) -> Crypto {
        let data: Url = create_url_crypto(function, symbol, market, self.get_api());
        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let crypto_helper: CryptoHelper =
            serde_json::from_str(body).expect("Cannot convert to CryptoHelper");
        crypto_helper.convert()
    }

    /// Method for exchanging currency value from one currency to another
    /// currency.
    ///
    /// # Example
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// assert_eq!(
    ///     api.exchange("BTC", "CNY").name_from().unwrap(),
    ///     String::from("Bitcoin")
    /// );
    /// ```
    #[must_use]
    pub fn exchange(&self, from_currency: &str, to_currency: &str) -> Exchange {
        let data: Url = format!(
            "{}CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}",
            LINK,
            from_currency,
            to_currency,
            self.get_api()
        )
        .parse()
        .expect("Failed to parse string to url");

        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let exchange_helper: ExchangeHelper =
            serde_json::from_str(body).expect("Cannot convert to Exchange");
        exchange_helper.convert()
    }

    /// Forex method for calling stock time series
    ///
    /// # Example
    /// ```
    /// use alpha_vantage::util::*;
    /// let api = alpha_vantage::set_api("demo");
    /// let forex = api.forex(
    ///     ForexFunction::Weekly,
    ///     "EUR",
    ///     "USD",
    ///     Interval::None,
    ///     OutputSize::None,
    /// );
    /// assert_eq!(forex.symbol_from().unwrap(), "EUR".to_string());
    /// ```
    #[must_use]
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
            self.get_api(),
        );
        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let forex_helper: ForexHelper =
            serde_json::from_str(body).expect("Cannot convert to ForexHelper");
        forex_helper.convert()
    }

    /// Method for returning Quote Struct
    /// # Example
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let quote = api.quote("MSFT");
    /// assert_eq!(quote.open().is_ok(), true);
    /// ```
    #[must_use]
    pub fn quote(&self, symbol: &str) -> Quote {
        let data: Url = format!(
            "{}GLOBAL_QUOTE&symbol={}&apikey={}",
            LINK,
            symbol,
            self.get_api()
        )
        .parse()
        .expect("Failed to parse quote str to URL");

        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let quote_helper: QuoteHelper =
            serde_json::from_str(body).expect("Cannot convert to Quote");
        quote_helper.convert()
    }

    /// Search method for searching keyword or company
    /// # Example
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let search = api.search("BA");
    /// assert_eq!(search.result().is_ok(), true);
    /// ```
    #[must_use]
    pub fn search(&self, keywords: &str) -> Search {
        let data: Url = format!(
            "{}SYMBOL_SEARCH&keywords={}&apikey={}",
            LINK,
            keywords,
            self.get_api()
        )
        .parse()
        .expect("Failed to parse search str to Url");
        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let search_helper: SearchHelper =
            serde_json::from_str(body).expect("Cannot convert to Search");
        search_helper.convert()
    }

    /// Method for returning out a sector data as struct
    /// # Example
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let sector = api.sector();
    /// assert_eq!(sector.information().is_ok(), true);
    /// ```
    #[must_use]
    pub fn sector(&self) -> Sector {
        let data: Url = format!("{}SECTOR&apikey={}", LINK, self.get_api())
            .parse()
            .expect("failed to parse sector str to Url");
        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let sector_helper: SectorHelper =
            serde_json::from_str(body).expect("cannot convert to SectorHelper");
        sector_helper.convert()
    }

    /// Stock time method for calling stock time series API
    /// # Example
    /// ```
    /// use alpha_vantage::util::*;
    /// let api = alpha_vantage::set_api("demo");
    /// let stock = api.stock_time(
    ///     StockFunction::Weekly,
    ///     "MSFT",
    ///     Interval::None,
    ///     OutputSize::None,
    /// );
    /// assert_eq!(stock.symbol().unwrap(), "MSFT".to_string());
    /// ```
    #[must_use]
    pub fn stock_time(
        &self,
        function: StockFunction,
        symbol: &str,
        interval: Interval,
        output_size: OutputSize,
    ) -> TimeSeries {
        let data: Url =
            create_url_time_series(function, symbol, interval, output_size, self.get_api());
        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let time_series_helper: TimeSeriesHelper =
            serde_json::from_str(body).expect("cannot convert to time series helper");
        time_series_helper.convert()
    }

    /// Technical indicator API caller method
    /// # Example
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let technical =
    ///     api.technical_indicator("SEMA", "MSFT", "1min", Some("open"), Some("10"), vec![]);
    /// assert_eq!(technical.data().is_ok(), true);
    /// ```
    #[must_use]
    pub fn technical_indicator(
        &self,
        function: &str,
        symbol: &str,
        interval: &str,
        series_type: Option<&str>,
        time_period: Option<&str>,
        temporary_value: Vec<TechnicalIndicator>,
    ) -> Indicator {
        let data = create_url_technical(
            function,
            symbol,
            interval,
            series_type,
            time_period,
            temporary_value,
            self.get_api(),
        );
        let body = &self
            .client
            .get(data)
            .send()
            .expect("failed to send out request")
            .text()
            .expect("failed to get out text from Response");
        let indicator_helper: IndicatorHelper =
            serde_json::from_str(body).expect("cannot convert to Indicator");
        indicator_helper.convert()
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

    #[test]
    fn set_api_from_env() {
        std::env::set_var("ALPHA_VANTAGE_KEY", "some_random_key");
        assert_eq!(
            super::APIKey::set_with_env("ALPHA_VANTAGE_KEY").get_api(),
            "some_random_key".to_string()
        );
    }

    #[test]
    fn test_set_get_timeout() {
        assert_eq!(super::APIKey::set_api("demo").get_timeout(), 30_u64);
        assert_eq!(
            super::APIKey::set_with_timeout("some_key", 45).get_timeout(),
            45_u64
        );
    }
}
