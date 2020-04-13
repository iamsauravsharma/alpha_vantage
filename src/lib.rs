#![warn(bare_trait_objects, missing_docs, unreachable_pub)]
#![deny(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::used_underscore_binding,
    clippy::needless_doctest_main
)]

//! Rust Client/Wrapper built for [Alphavantage][alpha_vantage_link] API.
//!
//! [alpha_vantage_link]: https://alphavantage.co

pub mod crypto;

pub mod exchange;

pub mod forex;

pub mod quote;

pub mod search;

pub mod sector;

pub mod stock_time;

pub mod technical_indicator;

/// Module for basic definition of user information like setting API and
/// requesting through that API
pub mod user;

/// Utility module declaring enum for basic function and parameters for
/// different API
pub mod util;

use self::user::APIKey;

/// Set API value which can be used for calling different module
///
/// ```
/// let api = alpha_vantage::set_api("some_key");
/// ```
#[must_use]
pub fn set_api(api: &str) -> APIKey {
    APIKey::set_api(api)
}

/// Set API value with timeout period
///
/// ```
/// let api_with_custom_timeout = alpha_vantage::set_with_timeout("your_api_key", 45);
/// ```
#[must_use]
pub fn set_with_timeout(api: &str, timeout: u64) -> APIKey {
    APIKey::set_with_timeout(api, timeout)
}

/// Set out API Key reading out environment variable
///
/// ```
/// std::env::set_var("KEY_NAME", "some_key");
/// let api_from_env = alpha_vantage::set_with_env("KEY_NAME");
/// assert_eq!(api_from_env.get_api(), "some_key");
/// ```
#[must_use]
pub fn set_with_env(env_name: &str) -> APIKey {
    APIKey::set_with_env(env_name)
}
