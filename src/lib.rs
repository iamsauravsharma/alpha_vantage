#![warn(bare_trait_objects, missing_docs, unreachable_pub)]
#![deny(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::used_underscore_binding,
    clippy::needless_doctest_main
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Rust Client/Wrapper built for [Alphavantage][alpha_vantage_link] API.
//!
//! [alpha_vantage_link]: https://alphavantage.co

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
/// Blocking module for basic definition of user information. To use this module
/// blocking feature need to be enabled
pub mod blocking;

pub mod crypto_rating;

pub mod crypto;

mod deserialize;

pub mod error;

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

/// Set API Key reading environment variable
///
/// ```
/// std::env::set_var("KEY_NAME", "some_key");
/// let api_from_env = alpha_vantage::set_from_env("KEY_NAME");
/// assert_eq!(api_from_env.get_api(), "some_key");
/// ```
#[must_use]
pub fn set_from_env(env_name: &str) -> APIKey {
    APIKey::set_from_env(env_name)
}

/// Set blocking API value which can be used for calling different module
///
/// ```
/// let api = alpha_vantage::blocking_set_api("some_key");
/// ```
#[must_use]
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub fn blocking_set_api(api: &str) -> self::blocking::APIKey {
    self::blocking::APIKey::set_api(api)
}

/// Set blocking API value with timeout period
///
/// ```
/// let api_with_custom_timeout = alpha_vantage::blocking_set_with_timeout("your_api_key", 45);
/// ```
#[must_use]
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub fn blocking_set_with_timeout(api: &str, timeout: u64) -> self::blocking::APIKey {
    self::blocking::APIKey::set_with_timeout(api, timeout)
}

/// Set blocking API Key reading environment variable
///
/// ```
/// std::env::set_var("KEY_NAME", "some_key");
/// let api_from_env = alpha_vantage::blocking_set_from_env("KEY_NAME");
/// assert_eq!(api_from_env.get_api(), "some_key");
/// ```
#[must_use]
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub fn blocking_set_from_env(env_name: &str) -> self::blocking::APIKey {
    self::blocking::APIKey::set_from_env(env_name)
}
