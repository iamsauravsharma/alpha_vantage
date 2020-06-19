//! Module for searching specific symbol or companies
//!
//! Looking for some specific symbols or companies? Trying to build a search box
//! similar to the one below?
//!
//! You can read about [Symbol][symbol_search] API and what it returns
//! on alphavantage documentation
//!
//! [symbol_search]: https://www.alphavantage.co/documentation/#symbolsearch

use crate::{
    deserialize::from_str,
    error::{Error, Result},
};
use serde::Deserialize;

/// struct for helping creation of search struct
#[derive(Debug, Deserialize)]
pub(crate) struct SearchHelper {
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "bestMatches")]
    matches: Option<Vec<DataValue>>,
}

impl SearchHelper {
    pub(crate) fn convert(self) -> Result<Search> {
        let mut search = Search::default();
        if let Some(information) = self.information {
            return Err(Error::AlphaVantageInformation(information));
        }
        search.matches = self.matches.unwrap();
        Ok(search)
    }
}

/// struct for storing search method data
#[derive(Default)]
pub struct Search {
    matches: Vec<DataValue>,
}

/// Struct which stores matches data for search keyword
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DataValue {
    #[serde(rename = "1. symbol")]
    symbol: String,
    #[serde(rename = "2. name")]
    name: String,
    #[serde(rename = "3. type")]
    stock_type: String,
    #[serde(rename = "4. region")]
    region: String,
    #[serde(rename = "5. marketOpen")]
    market_open: String,
    #[serde(rename = "6. marketClose")]
    market_close: String,
    #[serde(rename = "7. timezone")]
    time_zone: String,
    #[serde(rename = "8. currency")]
    currency: String,
    #[serde(rename = "9. matchScore", deserialize_with = "from_str")]
    match_score: f64,
}

impl DataValue {
    /// Return symbol
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let symbol = search.result()[0].symbol();
    ///     assert_eq!(symbol, "BA");
    /// }
    /// ```
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Return name for symbol
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let name = search.result()[0].name();
    ///     assert_eq!(name, "The Boeing Company");
    /// }
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return stock type
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let stock_type = search.result()[0].stock_type();
    ///     assert_eq!(stock_type, "Equity");
    /// }
    #[must_use]
    pub fn stock_type(&self) -> &str {
        &self.stock_type
    }

    /// Return region of search entry
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let region = search.result()[0].region();
    ///     assert_eq!(region, "United States");
    /// }
    #[must_use]
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Return market open time
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let market_open = search.result()[0].market_open();
    ///     assert_eq!(market_open, "09:30");
    /// }
    #[must_use]
    pub fn market_open(&self) -> &str {
        &self.market_open
    }

    /// Return market close time
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let market_close = search.result()[0].market_close();
    ///     assert_eq!(market_close, "16:00");
    /// }
    #[must_use]
    pub fn market_close(&self) -> &str {
        &self.market_close
    }

    /// Return time zone of symbol
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let time_zone = search.result()[0].time_zone();
    ///     assert_eq!(time_zone, "UTC-05");
    /// }
    #[must_use]
    pub fn time_zone(&self) -> &str {
        &self.time_zone
    }

    /// Return currency
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let currency = search.result()[0].currency();
    ///     assert_eq!(currency, "USD");
    /// }
    #[must_use]
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Return match score
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let search = api.search("BA").await.unwrap();
    ///     let match_score = search.result()[0].match_score();
    ///     assert_eq!(match_score, 1.0);
    /// }
    #[must_use]
    pub fn match_score(&self) -> f64 {
        self.match_score
    }
}

impl Search {
    /// Return result of search
    #[must_use]
    pub fn result(&self) -> &Vec<DataValue> {
        &self.matches
    }
}
