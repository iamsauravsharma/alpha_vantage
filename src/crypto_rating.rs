//! Module for getting crypto health rating provided by FCAS metric
//!
//! Fundamental Crypto Asset Score (FCAS) is a comparative metric used to assess
//! the fundamental health of crypto projects. The score is derived from the
//! interactivity between primary project life-cycle factors: User
//! Activity/Utility, Developer Behavior, and Market Maturity. Each crypto asset
//! is given a composite numerical score, 0-1000, and an associated rating as
//! follows:
//! ![Rating Image](https://www.alphavantage.co/static/img/fcas.svg)
//!
//! You can read about [Crypto Heath Rating][crypto_health_rating] API and what
//! it returns on alphavantage documentation
//!
//! [crypto_health_rating]: https://www.alphavantage.co/documentation/#crypto-ratings

use crate::{
    deserialize::from_str,
    error::{Error, Result},
};
use serde::Deserialize;

/// Struct Storing Health rating score
#[derive(Debug, Deserialize, Clone, Default)]
struct RatingScore {
    #[serde(rename = "1. symbol")]
    symbol: String,
    #[serde(rename = "2. name")]
    name: String,
    #[serde(rename = "3. fcas rating")]
    fcas_rating: String,
    #[serde(rename = "4. fcas score", deserialize_with = "from_str")]
    fcas_score: u16,
    #[serde(rename = "5. developer score", deserialize_with = "from_str")]
    developer_score: u16,
    #[serde(rename = "6. market maturity score", deserialize_with = "from_str")]
    market_maturity_score: u16,
    #[serde(rename = "7. utility score", deserialize_with = "from_str")]
    utility_score: u16,
    #[serde(rename = "8. last refreshed", deserialize_with = "from_str")]
    last_refreshed: String,
    #[serde(rename = "9. timezone", deserialize_with = "from_str")]
    time_zone: String,
}

/// Struct used for health index rating
#[derive(Default)]
pub struct CryptoRating {
    rating_score: RatingScore,
}

impl CryptoRating {
    /// Get symbol from which crypto rating was determined
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto_rating = api.crypto_rating("BTC").await.unwrap();
    ///     let symbol = crypto_rating.symbol();
    ///     assert_eq!(symbol, "BTC");
    /// }
    /// ```
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.rating_score.symbol
    }

    /// Get name for which crypto rating was determined
    ///
    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto_rating = api.crypto_rating("BTC").await.unwrap();
    ///     let name = crypto_rating.name();
    ///     assert_eq!(name, "Bitcoin");
    /// }
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.rating_score.name
    }

    /// Get time when crypto rating was last refreshed.
    #[must_use]
    pub fn refreshed_time(&self) -> &str {
        &self.rating_score.last_refreshed
    }

    /// Return time zone of last refreshed time

    /// ```
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let crypto_rating = api.crypto_rating("BTC").await.unwrap();
    ///     let time_zone = crypto_rating.time_zone();
    ///     assert_eq!(time_zone, "UTC");
    /// }
    /// ```
    #[must_use]
    pub fn time_zone(&self) -> &str {
        &self.rating_score.time_zone
    }

    /// Return fcas rating rank
    #[must_use]
    pub fn fcas_rating(&self) -> &str {
        &self.rating_score.fcas_rating
    }

    /// Return fcas score
    #[must_use]
    pub fn fcas_score(&self) -> u16 {
        self.rating_score.fcas_score
    }

    /// Return developer score
    #[must_use]
    pub fn developer_score(&self) -> u16 {
        self.rating_score.developer_score
    }

    /// Return market maturity score
    #[must_use]
    pub fn market_maturity_score(&self) -> u16 {
        self.rating_score.market_maturity_score
    }

    /// Return utility score
    #[must_use]
    pub fn utility_score(&self) -> u16 {
        self.rating_score.utility_score
    }
}

/// struct used for helping creation of crypto rating
#[derive(Debug, Deserialize)]
pub(crate) struct CryptoRatingHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Crypto Rating (FCAS)")]
    rating_score: Option<RatingScore>,
}

impl CryptoRatingHelper {
    pub(crate) fn convert(self) -> Result<CryptoRating> {
        let mut crypto_rating = CryptoRating::default();
        if let Some(information) = self.information {
            return Err(Error::AlphaVantageInformation(information));
        }
        if let Some(error_message) = self.error_message {
            return Err(Error::AlphaVantageErrorMessage(error_message));
        }
        crypto_rating.rating_score = self.rating_score.unwrap();
        Ok(crypto_rating)
    }
}
