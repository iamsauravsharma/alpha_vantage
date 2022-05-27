#![warn(bare_trait_objects, missing_docs, unreachable_pub)]
#![deny(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Rust Client/Wrapper built for [Alphavantage][alpha_vantage_link] API.
//!
//! [alpha_vantage_link]: https://alphavantage.co

/// Module for basic definition of user information like setting API and
/// requesting through that API
pub mod api;

/// Module which provides trait to implement own client as well as default
/// client in project
pub mod client;

pub mod crypto;

/// Module for custom url call
pub mod custom;

mod deserialize;

pub mod earning;

pub mod economic_indicator;

pub mod error;

pub mod exchange;

pub mod forex;

pub mod quote;

pub mod search;

pub mod sector;

pub mod stock_time;

pub mod technical_indicator;

pub use self::api::ApiClient;
use self::client::HttpClient;

/// Set API key using user selected or created client
///
/// ```
/// let api = alpha_vantage::set_api("some_key", reqwest::Client::new());
/// ```
#[must_use]
pub fn set_api<T>(api: &str, client: T) -> ApiClient
where
    T: HttpClient + 'static + Send + Sync,
{
    ApiClient::set_api(api, client)
}

/// Set Rapid API key using user selected or created client
///
/// ```
/// let api = alpha_vantage::set_rapid_api("some_key", reqwest::Client::new());
/// ```
#[must_use]
pub fn set_rapid_api<T>(api: &str, client: T) -> ApiClient
where
    T: HttpClient + 'static + Send + Sync,
{
    ApiClient::set_rapid_api(api, client)
}
