use serde::de::DeserializeOwned;

use crate::{
    client::HttpClient,
    crypto::{Crypto, CryptoHelper},
    crypto_rating::{CryptoRating, CryptoRatingHelper},
    earning::{Earning, EarningHelper},
    error::{Error, Result},
    exchange::{Exchange, ExchangeHelper},
    forex::{Forex, ForexHelper},
    quote::{Quote, QuoteHelper},
    search::{Search, SearchHelper},
    sector::{Sector, SectorHelper},
    stock_time::{TimeSeries, TimeSeriesHelper},
    technical_indicator::{Indicator, IndicatorHelper},
    utils::{
        CryptoFunction, ForexFunction, OutputSize, StockFunction, TechnicalIndicator,
        TechnicalIndicatorInterval, TimeSeriesInterval,
    },
};

const BASE_URL: &str = "https://www.alphavantage.co/";

/// Struct for initializing client which contains different method for API call
pub struct ApiClient {
    api: String,
    client: Box<dyn HttpClient>,
}

impl ApiClient {
    /// Method for initializing [ApiClient][ApiClient] struct using  user
    /// provided client
    ///
    /// ```
    /// use alpha_vantage::api::ApiClient;
    /// let api = ApiClient::set_api("some_key", surf::Client::new());
    /// ```
    #[must_use]
    pub fn set_api<T>(api: &str, client: T) -> Self
    where
        T: HttpClient + 'static,
    {
        Self {
            api: api.to_string(),
            client: Box::new(client),
        }
    }

    /// Method to get api key
    ///
    /// ```
    /// use alpha_vantage::api::ApiClient;
    /// let api = alpha_vantage::api::ApiClient::set_api("some_key", surf::Client::new());
    /// assert_eq!(api.get_api(), "some_key");
    /// ```
    #[must_use]
    pub fn get_api(&self) -> &str {
        &self.api
    }

    // Get json from api endpoint and create struct
    async fn get_json<T>(&self, path: String) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let full_path = format!("{}{}", BASE_URL, path);
        let string_output = self.client.get_output(full_path).await?;
        serde_json::from_str(&string_output).map_err(|_| Error::DecodeJsonToStruct)
    }

    /// Method for getting crypto health rating
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let crypto_rating = api.crypto_rating("BTC").await.unwrap();
    ///     assert_eq!(crypto_rating.symbol(), "BTC");
    ///     assert_eq!(crypto_rating.name(), "Bitcoin");
    ///     assert_eq!(crypto_rating.time_zone(), "UTC");
    /// }
    /// ```
    pub async fn crypto_rating(&self, symbol: &str) -> Result<CryptoRating> {
        let path = format!(
            "query?function=CRYPTO_RATING&symbol={}&apikey={}",
            symbol,
            self.get_api()
        );
        let crypto_rating_helper: CryptoRatingHelper = self.get_json(path).await?;
        crypto_rating_helper.convert()
    }

    /// Crypto method for calling cryptography function
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::utils::CryptoFunction::Daily, "BTC", "CNY")
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(crypto.digital_code(), "BTC");
    ///     assert_eq!(crypto.digital_name(), "Bitcoin");
    ///     assert_eq!(crypto.market_code(), "CNY");
    ///     assert_eq!(crypto.market_name(), "Chinese Yuan");
    /// }
    /// ```
    pub async fn crypto(
        &self,
        function: CryptoFunction,
        symbol: &str,
        market: &str,
    ) -> Result<Crypto> {
        let path = crate::crypto::create_url(function, symbol, market, self.get_api());
        let crypto_helper: CryptoHelper = self.get_json(path).await?;
        crypto_helper.convert()
    }

    /// Method for returning company earning
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let earning = api.earning("IBM").await.unwrap();
    ///     let symbol = earning.symbol();
    ///     assert_eq!(symbol, "IBM");
    /// }
    /// ```
    pub async fn earning(&self, symbol: &str) -> Result<Earning> {
        let path = format!(
            "query?function=EARNINGS&symbol={}&apikey={}",
            symbol,
            self.get_api()
        );
        let earning_helper: EarningHelper = self.get_json(path).await?;
        earning_helper.convert()
    }

    /// Method for exchanging currency value from one currency to another
    /// currency.
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let exchange = api.exchange("BTC", "CNY").await.unwrap();
    ///     assert_eq!(exchange.name_from(), "Bitcoin");
    ///     assert_eq!(exchange.code_from(), "BTC");
    ///     assert_eq!(exchange.name_to(), "Chinese Yuan");
    ///     assert_eq!(exchange.code_to(), "CNY");
    /// }
    /// ```
    pub async fn exchange(&self, from_currency: &str, to_currency: &str) -> Result<Exchange> {
        let path = format!(
            "query?function=CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}",
            from_currency,
            to_currency,
            self.get_api()
        );
        let exchange_helper: ExchangeHelper = self.get_json(path).await?;
        exchange_helper.convert()
    }

    /// Method for calling stock time series forex Api
    ///
    /// # Example
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::Weekly,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::None,
    ///             OutputSize::None,
    ///         )
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(forex.symbol_from(), "EUR");
    ///     assert_eq!(forex.symbol_to(), "USD");
    ///     assert!(forex.interval().is_none());
    /// }
    /// ```
    pub async fn forex(
        &self,
        function: ForexFunction,
        from_symbol: &str,
        to_symbol: &str,
        interval: TimeSeriesInterval,
        output_size: OutputSize,
    ) -> Result<Forex> {
        let path = crate::forex::create_url(
            function,
            from_symbol,
            to_symbol,
            interval,
            output_size,
            self.get_api(),
        );
        let forex_helper: ForexHelper = self.get_json(path).await?;
        forex_helper.convert()
    }

    /// Method for returning Quote Struct
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let quote = api.quote("MSFT").await.unwrap();
    ///     let symbol = quote.symbol();
    ///     assert_eq!(symbol, "MSFT");
    /// }
    /// ```
    pub async fn quote(&self, symbol: &str) -> Result<Quote> {
        let path = format!(
            "query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            symbol,
            self.get_api()
        );
        let quote_helper: QuoteHelper = self.get_json(path).await?;
        quote_helper.convert()
    }

    /// Method for searching keyword or company
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let search = api.search("BA").await.unwrap();
    ///     let first_search_result = &search.result()[0];
    ///     assert_eq!(first_search_result.symbol(), "BA");
    ///     assert_eq!(first_search_result.name(), "Boeing Company");
    ///     assert_eq!(first_search_result.stock_type(), "Equity");
    ///     assert_eq!(first_search_result.region(), "United States");
    ///     assert_eq!(first_search_result.currency(), "USD");
    ///     assert_eq!(first_search_result.match_score(), 1.0);
    /// }
    /// ```
    pub async fn search(&self, keywords: &str) -> Result<Search> {
        let path = format!(
            "query?function=SYMBOL_SEARCH&keywords={}&apikey={}",
            keywords,
            self.get_api()
        );
        let search_helper: SearchHelper = self.get_json(path).await?;
        search_helper.convert()
    }

    /// Method for returning a sector data as struct
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let sector = api.sector().await.unwrap();
    ///     assert_eq!(
    ///         sector.information(),
    ///         "US Sector Performance (realtime & historical)"
    ///     );
    /// }
    /// ```
    pub async fn sector(&self) -> Result<Sector> {
        let path = format!("query?function=SECTOR&apikey={}", self.get_api());
        let sector_helper: SectorHelper = self.get_json(path).await?;
        sector_helper.convert()
    }

    /// Method for calling stock time series API
    /// # Example
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let stock = api
    ///         .stock_time(
    ///             StockFunction::Weekly,
    ///             "MSFT",
    ///             TimeSeriesInterval::None,
    ///             OutputSize::None,
    ///         )
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(stock.symbol(), "MSFT");
    ///     assert!(stock.interval().is_none());
    /// }
    /// ```
    pub async fn stock_time(
        &self,
        function: StockFunction,
        symbol: &str,
        interval: TimeSeriesInterval,
        output_size: OutputSize,
    ) -> Result<TimeSeries> {
        let path =
            crate::stock_time::create_url(function, symbol, interval, output_size, self.get_api());
        let time_series_helper: TimeSeriesHelper = self.get_json(path).await?;
        time_series_helper.convert()
    }

    /// Method for technical indicator API
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", surf::Client::new());
    ///     let technical = api
    ///         .technical_indicator(
    ///             "SMA",
    ///             "IBM",
    ///             alpha_vantage::utils::TechnicalIndicatorInterval::Weekly,
    ///             Some(10),
    ///             Some("open"),
    ///             vec![],
    ///         )
    ///         .await;
    ///     assert!(technical.is_ok());
    /// }
    /// ```
    pub async fn technical_indicator(
        &self,
        function: &str,
        symbol: &str,
        interval: TechnicalIndicatorInterval,
        time_period: Option<u64>,
        series_type: Option<&str>,
        extras: Vec<TechnicalIndicator>,
    ) -> Result<Indicator> {
        let path = crate::technical_indicator::create_url(
            function,
            symbol,
            interval,
            time_period,
            series_type,
            extras,
            self.get_api(),
        );
        let indicator_helper: IndicatorHelper = self.get_json(path).await?;
        indicator_helper.convert()
    }
}

// Mod for unit testing
#[cfg(test)]
mod test {

    #[test]
    fn test_get_api() {
        let api = super::ApiClient::set_api("secret_key", surf::Client::new());
        assert_eq!(api.get_api(), "secret_key".to_string());
    }
}
