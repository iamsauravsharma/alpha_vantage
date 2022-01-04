//! Module for sector
//!
//! This API returns the realtime and historical sector performances calculated
//! from S&P500 incumbents.
//!
//! You can read about [Sector][sector] API and what it returns
//! on alphavantage documentation
//!
//! [sector]: https://www.alphavantage.co/documentation/#sector

use std::collections::HashMap;

use serde::Deserialize;

use crate::api::ApiClient;
use crate::error::{detect_common_helper_error, Error, Result};

/// Stores Metadata
#[derive(Deserialize, Clone, Default)]
struct MetaData {
    #[serde(rename = "Information")]
    information: String,
    #[serde(rename = "Last Refreshed")]
    last_refreshed: String,
}

/// Store Sector data
#[derive(Default, Clone)]
pub struct Data {
    rank: String,
    utilities: f64,
    health_care: f64,
    information_technology: f64,
    industrials: f64,
    real_estate: f64,
    consumer_staples: f64,
    consumer_discretionary: f64,
    financials: f64,
    communication_services: f64,
    materials: f64,
    energy: f64,
}

impl Data {
    /// Return rank
    #[must_use]
    pub fn rank(&self) -> &str {
        &self.rank
    }

    /// Return utilities score
    #[must_use]
    pub fn utilities(&self) -> f64 {
        self.utilities
    }

    /// Return health care score
    #[must_use]
    pub fn health_care(&self) -> f64 {
        self.health_care
    }

    /// Return information technology score
    #[must_use]
    pub fn information_technology(&self) -> f64 {
        self.information_technology
    }

    /// Return industrials scores
    #[must_use]
    pub fn industrials(&self) -> f64 {
        self.industrials
    }

    /// Return real estate value
    #[must_use]
    pub fn real_estate(&self) -> f64 {
        self.real_estate
    }

    /// Return consumer staples value
    #[must_use]
    pub fn consumer_staples(&self) -> f64 {
        self.consumer_staples
    }

    /// Return consumer discretionary score
    #[must_use]
    pub fn consumer_discretionary(&self) -> f64 {
        self.consumer_discretionary
    }

    /// Return financials score
    #[must_use]
    pub fn financials(&self) -> f64 {
        self.financials
    }

    /// Gives value of communication services
    #[must_use]
    pub fn communication_services(&self) -> f64 {
        self.communication_services
    }

    /// Gives materials value
    #[must_use]
    pub fn materials(&self) -> f64 {
        self.materials
    }

    /// Gives out energy data
    #[must_use]
    pub fn energy(&self) -> f64 {
        self.energy
    }
}

/// Stores sector data
#[derive(Default)]
pub struct Sector {
    meta_data: MetaData,
    data: Vec<Data>,
}

impl Sector {
    /// Return sector information
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let sector = api.sector().json().await.unwrap();
    ///     let information = sector.information();
    ///     assert_eq!(information, "US Sector Performance (realtime & historical)");
    /// }
    /// ```
    #[must_use]
    pub fn information(&self) -> &str {
        self.return_meta_data_val("information")
    }

    /// Return last refreshed time
    #[must_use]
    pub fn last_refreshed(&self) -> &str {
        self.return_meta_data_val("last refreshed")
    }

    /// Return vector of data
    #[must_use]
    pub fn data(&self) -> &Vec<Data> {
        &self.data
    }

    /// Return metadata value
    fn return_meta_data_val(&self, name: &str) -> &str {
        match name {
            "information" => &self.meta_data.information,
            "last refreshed" => &self.meta_data.last_refreshed,
            _ => "",
        }
    }
}

/// struct for helping out sector
#[derive(Deserialize)]
pub(crate) struct SectorHelper {
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<MetaData>,
    #[serde(flatten)]
    data: Option<HashMap<String, HashMap<String, String>>>,
}

impl SectorHelper {
    /// Convert `SectorHelper` to `Sector`
    pub(crate) fn convert(self) -> Result<Sector> {
        let mut sector = Sector::default();
        detect_common_helper_error(self.information, self.error_message, self.note)?;
        if self.meta_data.is_none() || self.data.is_none() {
            return Err(Error::EmptyResponse);
        }
        sector.meta_data = self.meta_data.unwrap();
        let mut final_data = Vec::new();
        for (key, val) in &self.data.unwrap() {
            let mut data = Data::default();
            match key.as_str() {
                "Rank A: Real-Time Performance" => data.rank = "real-time".to_string(),
                "Rank B: 1 Day Performance" => data.rank = "1-day".to_string(),
                "Rank C: 5 Day Performance" => data.rank = "5-day".to_string(),
                "Rank D: 1 Month Performance" => data.rank = "1-month".to_string(),
                "Rank E: 3 Month Performance" => data.rank = "3-month".to_string(),
                "Rank F: Year-to-Date (YTD) Performance" => data.rank = "year-to-date".to_string(),
                "Rank G: 1 Year Performance" => data.rank = "1-year".to_string(),
                "Rank H: 3 Year Performance" => data.rank = "3-year".to_string(),
                "Rank I: 5 Year Performance" => data.rank = "5-year".to_string(),
                "Rank J: 10 Year Performance" => data.rank = "10-year".to_string(),
                _ => data.rank = "".to_string(),
            }
            for (key, val) in val.iter() {
                match key.as_str() {
                    "Utilities" => data.utilities = convert_str_percent_f64(val),
                    "Health Care" => data.health_care = convert_str_percent_f64(val),
                    "Information Technology" => {
                        data.information_technology = convert_str_percent_f64(val);
                    }
                    "Industrials" => data.industrials = convert_str_percent_f64(val),
                    "Real Estate" => data.real_estate = convert_str_percent_f64(val),
                    "Consumer Staples" => data.consumer_staples = convert_str_percent_f64(val),
                    "Consumer Discretionary" => {
                        data.consumer_discretionary = convert_str_percent_f64(val);
                    }
                    "Financials" => data.financials = convert_str_percent_f64(val),
                    "Communication Services" => {
                        data.communication_services = convert_str_percent_f64(val);
                    }
                    "Materials" => data.materials = convert_str_percent_f64(val),
                    "Energy" => data.energy = convert_str_percent_f64(val),
                    _ => {}
                }
            }
            final_data.push(data);
        }
        sector.data = final_data;
        Ok(sector)
    }
}

// convert str which has percent form to f64 val
fn convert_str_percent_f64(val: &str) -> f64 {
    let mut s = val.to_owned();
    s.pop();
    s.trim().parse::<f64>().unwrap()
}

/// Builder to create new Sector
pub struct SectorBuilder<'a> {
    api_client: &'a ApiClient,
}

impl<'a> SectorBuilder<'a> {
    /// Create new sector builder from `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient) -> Self {
        Self { api_client }
    }

    fn create_url() -> String {
        String::from("query?function=SECTOR")
    }

    /// Returns JSON data struct
    ///
    /// # Errors
    /// Raise error if data obtained cannot be properly converted to struct or
    /// API returns any 4 possible known errors
    pub async fn json(&self) -> Result<Sector> {
        let url = Self::create_url();
        let sector_helper: SectorHelper = self.api_client.get_json(url).await?;
        sector_helper.convert()
    }
}
