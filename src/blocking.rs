use crate::{
    crypto::{create_url as create_url_crypto, Crypto, CryptoHelper},
    crypto_rating::{CryptoRating, CryptoRatingHelper},
    error::Result,
    exchange::{Exchange, ExchangeHelper},
    forex::{create_url as create_url_forex, Forex, ForexHelper},
    quote::{Quote, QuoteHelper},
    search::{Search, SearchHelper},
    sector::{Sector, SectorHelper},
    stock_time::{create_url as create_url_time_series, TimeSeries, TimeSeriesHelper},
    technical_indicator::{create_url as create_url_technical, Indicator, IndicatorHelper},
    util::{
        CryptoFunction, ForexFunction, OutputSize, StockFunction, TechnicalIndicator,
        TechnicalIndicatorInterval, TimeSeriesInterval,
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
    /// use alpha_vantage::blocking::APIKey;
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
    /// use alpha_vantage::blocking::APIKey;
    /// let api_with_custom_timeout = APIKey::set_with_timeout("your_api_key", 45);
    /// ```
    #[must_use]
    pub fn set_with_timeout(api: &str, timeout: u64) -> Self {
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(timeout))
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
    /// use alpha_vantage::blocking::APIKey;
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
    /// use alpha_vantage::blocking::APIKey;
    /// let mut api = alpha_vantage::blocking::APIKey::set_api("some_key");
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
    /// use alpha_vantage::blocking::APIKey;
    /// let api = alpha_vantage::blocking::APIKey::set_api("some_key");
    /// assert_eq!(api.get_api(), "some_key");
    /// ```
    #[must_use]
    pub fn get_api(&self) -> &str {
        &self.api
    }

    /// Get API timeout period
    ///
    /// ```
    /// use alpha_vantage::blocking::APIKey;
    /// let api_with_custom_timeout = APIKey::set_with_timeout("your_api_key", 45);
    /// assert_eq!(api_with_custom_timeout.get_timeout(), 45_u64);
    /// ```
    #[must_use]
    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }
    /// Method for getting crypto health rating
    ///
    /// # Example
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// assert_eq!(api.crypto_rating("BTC").unwrap().name(), "Bitcoin");
    /// ```
    pub fn crypto_rating(&self, symbol: &str) -> Result<CryptoRating> {
        let data: Url = format!(
            "{}CRYPTO_RATING&symbol={}&apikey={}",
            LINK,
            symbol,
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
        let crypto_rating_helper: CryptoRatingHelper =
            serde_json::from_str(body).expect("Cannot convert to crypto rating");
        crypto_rating_helper.convert()
    }

    /// Crypto method for calling cryptography function
    ///
    /// # Example
    /// ```
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// let crypto = api
    ///     .crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY")
    ///     .unwrap();
    /// let digital_name = crypto.digital_name();
    /// assert_eq!(digital_name, "Bitcoin");
    /// ```
    pub fn crypto(&self, function: CryptoFunction, symbol: &str, market: &str) -> Result<Crypto> {
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
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// assert_eq!(api.exchange("BTC", "CNY").unwrap().name_from(), "Bitcoin");
    /// ```
    pub fn exchange(&self, from_currency: &str, to_currency: &str) -> Result<Exchange> {
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
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// let forex = api.forex(
    ///     alpha_vantage::util::ForexFunction::Weekly,
    ///     "EUR",
    ///     "USD",
    ///     alpha_vantage::util::TimeSeriesInterval::None,
    ///     alpha_vantage::util::OutputSize::None,
    /// );
    /// assert_eq!(forex.unwrap().symbol_from(), "EUR");
    /// ```
    pub fn forex(
        &self,
        function: ForexFunction,
        from_symbol: &str,
        to_symbol: &str,
        interval: TimeSeriesInterval,
        output_size: OutputSize,
    ) -> Result<Forex> {
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
    ///
    /// # Example
    /// ```
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// let quote = api.quote("MSFT").unwrap();
    /// let symbol = quote.symbol();
    /// assert_eq!(symbol, "MSFT");
    /// ```
    pub fn quote(&self, symbol: &str) -> Result<Quote> {
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
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// let search = api.search("BA").unwrap();
    /// assert_eq!(search.result()[0].symbol(), "BA");
    /// ```
    pub fn search(&self, keywords: &str) -> Result<Search> {
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
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// let sector = api.sector().unwrap();
    /// assert_eq!(
    ///     sector.information(),
    ///     "US Sector Performance (realtime & historical)"
    /// );
    /// ```
    pub fn sector(&self) -> Result<Sector> {
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
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// let stock = api
    ///     .stock_time(
    ///         alpha_vantage::util::StockFunction::Weekly,
    ///         "MSFT",
    ///         alpha_vantage::util::TimeSeriesInterval::None,
    ///         alpha_vantage::util::OutputSize::None,
    ///     )
    ///     .unwrap();
    /// assert_eq!(stock.symbol(), "MSFT".to_string());
    /// ```
    pub fn stock_time(
        &self,
        function: StockFunction,
        symbol: &str,
        interval: TimeSeriesInterval,
        output_size: OutputSize,
    ) -> Result<TimeSeries> {
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
    /// let api = alpha_vantage::blocking::APIKey::set_api("demo");
    /// let technical = api.technical_indicator(
    ///     "SMA",
    ///     "IBM",
    ///     alpha_vantage::util::TechnicalIndicatorInterval::Weekly,
    ///     Some(10),
    ///     Some("open"),
    ///     vec![],
    /// );
    /// assert_eq!(technical.is_ok(), true);
    /// ```
    pub fn technical_indicator(
        &self,
        function: &str,
        symbol: &str,
        interval: TechnicalIndicatorInterval,
        time_period: Option<u64>,
        series_type: Option<&str>,
        temporary_value: Vec<TechnicalIndicator>,
    ) -> Result<Indicator> {
        let data = create_url_technical(
            function,
            symbol,
            interval,
            time_period,
            series_type,
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
