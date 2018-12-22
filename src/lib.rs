//! Rust Client/Wrapper built for [Alphavantage][alpha_vantage_link] API.
//!
//! [alpha_vantage_link]: https://alphavantage.co

/// Module for crypto real time data
pub mod crypto;

/// Module for exchange currency (both digital & physical currency exchange)
pub mod exchange;

/// Module for Forex realtime and historical data
pub mod forex;

/// Module for returning latest price and volume information
pub mod quote;

/// Module for searching specific symbol or companies
pub mod search;

/// Module for sector
pub mod sector;

/// Stock time series module
pub mod stock_time;

/// Module for Technical Indicator
pub mod technical_indicator;

/// Module for basic definition of User information like setting API
pub mod user;

/// Utility module declaring enum for basic use
pub mod util;

use self::user::APIKey;

/// Set api value
pub fn set_api(api: &str) -> APIKey {
    APIKey::set_api(api)
}
