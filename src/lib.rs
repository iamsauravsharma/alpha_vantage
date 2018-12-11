pub mod crypto;

/// Module for exchange currency (both digital & physical currency exchange)
pub mod exchange;

/// Module for Forex realtime and historical data
pub mod forex;

/// Module for returning latest price and volume information
pub mod quote;

/// Module for searching specific symbol or companies
pub mod search;

pub mod sector;

pub mod technical_indicator;

/// Stock time series module
pub mod time_series;

/// Module for basic definition of User information like setting API
pub mod user;

/// Utility module declaring enum for basic use
pub mod util;

use self::user::APIKey;

/// Set api value
pub fn set_api(api: &str) -> APIKey {
    APIKey::set_api(api)
}
