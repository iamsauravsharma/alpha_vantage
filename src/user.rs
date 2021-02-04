use surf::{Client, Url};

use crate::{
    crypto::{create_url as create_url_crypto, Crypto, CryptoHelper},
    crypto_rating::{CryptoRating, CryptoRatingHelper},
    earning::{Earning, EarningHelper},
    error::Result,
    exchange::{Exchange, ExchangeHelper},
    forex::{create_url as create_url_forex, Forex, ForexHelper},
    income_statement::{IncomeStatement, IncomeStatementHelper},
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
const BASE_URL: &str = "https://www.alphavantage.co/";

/// Struct for initializing api key value as well as contain different method
/// for API call
pub struct APIKey {
    api: String,
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
        let mut client = Client::new();
        client.set_base_url(Url::parse(BASE_URL).unwrap());
        Self {
            api: api.to_string(),
            client,
        }
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

    /// Method for getting crypto health rating
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     assert_eq!(api.crypto_rating("BTC").await.unwrap().name(), "Bitcoin");
    /// }
    /// ```
    pub async fn crypto_rating(&self, symbol: &str) -> Result<CryptoRating> {
        let path = format!(
            "query?function=CRYPTO_RATING&symbol={}&apikey={}",
            symbol,
            self.get_api()
        );
        let crypto_rating_helper: CryptoRatingHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        crypto_rating_helper.convert()
    }

    /// Crypto method for calling cryptography function
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto = api
    ///         .crypto(alpha_vantage::util::CryptoFunction::Daily, "BTC", "CNY")
    ///         .await
    ///         .unwrap();
    ///     let digital_name = crypto.digital_name();
    ///     assert_eq!(digital_name, "Bitcoin");
    /// }
    /// ```
    pub async fn crypto(
        &self,
        function: CryptoFunction,
        symbol: &str,
        market: &str,
    ) -> Result<Crypto> {
        let path = create_url_crypto(function, symbol, market, self.get_api());
        let crypto_helper: CryptoHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        crypto_helper.convert()
    }

    /// Earning method for returning company earning
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
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
        let earning_helper: EarningHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        earning_helper.convert()
    }

    /// Method for exchanging currency value from one currency to another
    /// currency.
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     assert_eq!(
    ///         api.exchange("BTC", "CNY").await.unwrap().name_from(),
    ///         "Bitcoin"
    ///     );
    /// }
    /// ```
    pub async fn exchange(&self, from_currency: &str, to_currency: &str) -> Result<Exchange> {
        let path = format!(
            "query?function=CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}",
            from_currency,
            to_currency,
            self.get_api()
        );
        let exchange_helper: ExchangeHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        exchange_helper.convert()
    }

    /// Forex method for calling stock time series
    ///
    /// # Example
    /// ```
    /// use alpha_vantage::util::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::Weekly,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::None,
    ///             OutputSize::None,
    ///         )
    ///         .await;
    ///     assert_eq!(forex.unwrap().symbol_from(), "EUR");
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
        let path = create_url_forex(
            function,
            from_symbol,
            to_symbol,
            interval,
            output_size,
            self.get_api(),
        );
        let forex_helper: ForexHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        forex_helper.convert()
    }

    /// Method for returning income statement struct
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let income_statement = api.income_statement("IBM").await.unwrap();
    ///     let symbol = income_statement.symbol();
    ///     assert_eq!(symbol, "IBM");
    /// }
    /// ```
    pub async fn income_statement(&self, symbol: &str) -> Result<IncomeStatement> {
        let path = format!(
            "query?function=INCOME_STATEMENT&symbol={}&apikey={}",
            symbol,
            self.get_api()
        );
        let income_statement_helper: IncomeStatementHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        income_statement_helper.convert()
    }

    /// Method for returning Quote Struct
    ///
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
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
        let quote_helper: QuoteHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        quote_helper.convert()
    }

    /// Search method for searching keyword or company
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     assert_eq!(search.result()[0].symbol(), "BA");
    /// }
    /// ```
    pub async fn search(&self, keywords: &str) -> Result<Search> {
        let path = format!(
            "query?function=SYMBOL_SEARCH&keywords={}&apikey={}",
            keywords,
            self.get_api()
        );
        let search_helper: SearchHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        search_helper.convert()
    }

    /// Method for returning a sector data as struct
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let sector = api.sector().await.unwrap();
    ///     assert_eq!(
    ///         sector.information(),
    ///         "US Sector Performance (realtime & historical)"
    ///     );
    /// }
    /// ```
    pub async fn sector(&self) -> Result<Sector> {
        let path = format!("query?function=SECTOR&apikey={}", self.get_api());
        let sector_helper: SectorHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        sector_helper.convert()
    }

    /// Stock time method for calling stock time series API
    /// # Example
    /// ```
    /// use alpha_vantage::util::*;
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let stock = api
    ///         .stock_time(
    ///             StockFunction::Weekly,
    ///             "MSFT",
    ///             TimeSeriesInterval::None,
    ///             OutputSize::None,
    ///         )
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(stock.symbol(), "MSFT".to_string());
    /// }
    /// ```
    pub async fn stock_time(
        &self,
        function: StockFunction,
        symbol: &str,
        interval: TimeSeriesInterval,
        output_size: OutputSize,
    ) -> Result<TimeSeries> {
        let path = create_url_time_series(function, symbol, interval, output_size, self.get_api());
        let time_series_helper: TimeSeriesHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        time_series_helper.convert()
    }

    /// Technical indicator API caller method
    /// # Example
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let technical = api
    ///         .technical_indicator(
    ///             "SMA",
    ///             "IBM",
    ///             alpha_vantage::util::TechnicalIndicatorInterval::Weekly,
    ///             Some(10),
    ///             Some("open"),
    ///             vec![],
    ///         )
    ///         .await;
    ///     assert_eq!(technical.is_ok(), true);
    /// }
    /// ```
    pub async fn technical_indicator(
        &self,
        function: &str,
        symbol: &str,
        interval: TechnicalIndicatorInterval,
        time_period: Option<u64>,
        series_type: Option<&str>,
        temporary_value: Vec<TechnicalIndicator>,
    ) -> Result<Indicator> {
        let path = create_url_technical(
            function,
            symbol,
            interval,
            time_period,
            series_type,
            temporary_value,
            self.get_api(),
        );
        let indicator_helper: IndicatorHelper = self
            .client
            .get(path)
            .recv_json()
            .await
            .expect("fail to get json");
        indicator_helper.convert()
    }
}

// Mod for unit testing
#[cfg(test)]
mod test {
    #[test]
    // Testing get api and set api function
    fn test_get_api() {
        assert_eq!(
            super::APIKey::set_api("secret_key").get_api(),
            "secret_key".to_string()
        );
    }
}
