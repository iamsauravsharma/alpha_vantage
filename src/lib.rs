#![warn(bare_trait_objects, missing_docs, unreachable_pub)]
#![deny(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Rust Client/Wrapper built for [Alphavantage][alpha_vantage_link] API.
//!
//! [alpha_vantage_link]: https://alphavantage.co

/// Module for basic definition of user information like setting API and
/// requesting through that API
pub mod api;

/// Module which provides trait to implement own client as well as default
/// client in project
pub mod client;

pub mod crypto_rating;

pub mod crypto;

mod deserialize;

pub mod earning;

pub mod error;

pub mod exchange;

pub mod forex;

pub mod quote;

pub mod search;

pub mod sector;

pub mod stock_time;

pub mod technical_indicator;

/// Utility module declaring enum for basic function and parameters for
/// different API
pub mod utils;

use self::{api::ApiClient, client::HttpClient};

/// Set API key using user selected or created client
///
/// ```
/// let api = alpha_vantage::set_api("some_key", surf::Client::new());
/// ```
#[must_use]
pub fn set_api<T>(api: &str, client: T) -> ApiClient
where
    T: HttpClient + 'static,
{
    ApiClient::set_api(api, client)
}
