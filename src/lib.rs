#![warn(missing_docs, unreachable_pub)]
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

pub mod stock_time;

pub mod technical_indicator;

pub mod vec_trait;

pub use self::api::ApiClient;
use self::client::HttpClient;

/// Set API key using user selected or created client
///
/// ```
/// let api = alpha_vantage::set_api("some_key", reqwest::Client::new());
/// ```
#[must_use]
pub fn set_api<S, T>(api: S, client: T) -> ApiClient
where
    S: Into<String>,
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
pub fn set_rapid_api<S, T>(api: S, client: T) -> ApiClient
where
    S: Into<String>,
    T: HttpClient + 'static + Send + Sync,
{
    ApiClient::set_rapid_api(api, client)
}

/// Create json data struct
macro_rules! json_data_struct {
    ($output:ident, $helper:ident) => {
        /// Returns JSON data
        ///
        /// # Errors
        /// Raise error if data obtained cannot be properly converted to struct or
        /// API returns any 4 possible known errors
        pub async fn json(&self) -> Result<$output> {
            let url = self.create_url();
            let helper: $helper = self.api_client.get_json(&url).await?;
            helper.convert()
        }
    };
}

pub(crate) use json_data_struct;
