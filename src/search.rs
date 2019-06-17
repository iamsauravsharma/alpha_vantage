//! Module for searching specific symbol or companies
//!
//! Looking for some specific symbols or companies? Trying to build a search box
//! similar to the one below?
//!
//! You can read about [Symbol][symbol_search] API and what it returns
//! on alphavantage documentation
//!
//! [symbol_search]: https://www.alphavantage.co/documentation/#symbolsearch

use crate::user::APIKey;
use serde_derive::Deserialize;

/// struct for storing search method data
#[derive(Debug, Deserialize)]
pub struct Search {
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "bestMatches")]
    matches: Option<Vec<DataValue>>,
}

/// Struct which stores matches data for search keyword
#[derive(Debug, Clone, Deserialize)]
pub struct DataValue {
    #[serde(rename = "1. symbol")]
    symbol: String,
    #[serde(rename = "2. name")]
    name: String,
    #[serde(rename = "3. type")]
    data_type: String,
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
    #[serde(rename = "9. matchScore")]
    match_score: String,
}

impl DataValue {
    /// Return symbol
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Return name for symbol
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return data type
    pub fn data_type(&self) -> &str {
        &self.data_type
    }

    /// Return region of search entry
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Return open value
    pub fn market_open(&self) -> &str {
        &self.market_open
    }

    /// Return close value
    pub fn market_close(&self) -> &str {
        &self.market_close
    }

    /// Return time zone of symbol
    pub fn time_zone(&self) -> &str {
        &self.time_zone
    }

    /// Return currency
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Return match score
    pub fn match_score(&self) -> f64 {
        self.match_score.trim().parse::<f64>().unwrap()
    }
}

impl Search {
    /// Return result of search
    pub fn result(&self) -> Result<Vec<DataValue>, &str> {
        if let Some(entry) = &self.matches {
            Ok(entry.to_vec())
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }
}

/// Function used to create a [Search][Search] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub fn search(keyword: &str, api_data: (&str, Option<u64>)) -> Search {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.search(keyword)
}
