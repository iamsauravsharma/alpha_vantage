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

/// Module for basic definition of user information like setting API and
/// requesting through that API
pub mod api;

pub mod crypto_rating;

pub mod crypto;

mod deserialize;

pub mod earning;

pub mod error;

pub mod exchange;

pub mod forex;

pub mod income_statement;

pub mod quote;

pub mod search;

pub mod sector;

pub mod stock_time;

pub mod technical_indicator;

/// Utility module declaring enum for basic function and parameters for
/// different API
pub mod utils;

use self::api::APIClient;

/// Set API value which can be used for calling different module
///
/// ```
/// let api = alpha_vantage::set_api("some_key");
/// ```
#[must_use]
pub fn set_api(api: &str) -> APIClient {
    APIClient::set_api(api)
}
