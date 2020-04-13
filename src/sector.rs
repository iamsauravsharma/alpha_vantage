//! Module for sector
//!
//! This API returns the realtime and historical sector performances calculated
//! from S&P500 incumbents.
//!
//! You can read about [Sector][sector] API and what it returns
//! on alphavantage documentation
//!
//! [sector]: https://www.alphavantage.co/documentation/#sector

use crate::user::APIKey;
use serde::Deserialize;
use std::collections::HashMap;

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
    utilities: String,
    health_care: String,
    information_technology: String,
    industrials: String,
    real_estate: String,
    consumer_staples: String,
    consumer_discretionary: String,
    financials: String,
    communication_services: String,
    materials: String,
    energy: String,
}

impl Data {
    /// Return rank
    #[must_use]
    pub fn rank(&self) -> &str {
        &self.rank
    }

    /// Return utilities score
    #[must_use]
    pub fn utilities(&self) -> &str {
        &self.utilities
    }

    /// Return health care score
    #[must_use]
    pub fn health_care(&self) -> &str {
        &self.health_care
    }

    /// Return out information technology
    #[must_use]
    pub fn information_technology(&self) -> &str {
        &self.information_technology
    }

    /// Return industrials scores
    #[must_use]
    pub fn industrials(&self) -> &str {
        &self.industrials
    }

    /// Return out real estate value
    #[must_use]
    pub fn real_estate(&self) -> &str {
        &self.real_estate
    }

    /// Return consumer staples value
    #[must_use]
    pub fn consumer_staples(&self) -> &str {
        &self.consumer_staples
    }

    /// Return out value for consumer discretionary
    #[must_use]
    pub fn consumer_discretionary(&self) -> &str {
        &self.consumer_discretionary
    }

    /// Return out for financials
    #[must_use]
    pub fn financials(&self) -> &str {
        &self.financials
    }

    /// Gives value of communication services
    #[must_use]
    pub fn communication_services(&self) -> &str {
        &self.communication_services
    }

    /// Gives materials value
    #[must_use]
    pub fn materials(&self) -> &str {
        &self.materials
    }

    /// Gives out energy data
    #[must_use]
    pub fn energy(&self) -> &str {
        &self.energy
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
    /// use tokio::prelude::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let sector = api.sector().await.unwrap();
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
    #[serde(rename = "Meta Data")]
    meta_data: Option<MetaData>,
    #[serde(flatten)]
    data: Option<HashMap<String, HashMap<String, String>>>,
}

impl SectorHelper {
    /// Convert out [SectorHelper][SectorHelper] to [Sector][Sector]
    pub(crate) fn convert(self) -> Result<Sector, String> {
        let mut sector = Sector::default();
        if let Some(information) = self.information {
            return Err(information);
        }
        if let Some(error_message) = self.error_message {
            return Err(error_message);
        }
        sector.meta_data = self.meta_data.unwrap();
        if let Some(temp_data) = self.data {
            let mut final_data = Vec::new();
            for (key, val) in &temp_data {
                let mut data = Data::default();
                match key.as_str() {
                    "Rank A: Real-Time Performance" => data.rank = "real-time".to_string(),
                    "Rank B: 1 Day Performance" => data.rank = "1-day".to_string(),
                    "Rank C: 5 Day Performance" => data.rank = "5-day".to_string(),
                    "Rank D: 1 Month Performance" => data.rank = "1-month".to_string(),
                    "Rank E: 3 Month Performance" => data.rank = "3-month".to_string(),
                    "Rank F: Year-to-Date (YTD) Performance" => {
                        data.rank = "year-to-date".to_string()
                    }
                    "Rank G: 1 Year Performance" => data.rank = "1-year".to_string(),
                    "Rank H: 3 Year Performance" => data.rank = "3-year".to_string(),
                    "Rank I: 5 Year Performance" => data.rank = "5-year".to_string(),
                    "Rank J: 10 Year Performance" => data.rank = "10-year".to_string(),
                    _ => data.rank = "".to_string(),
                }
                for (key, val) in val.iter() {
                    match key.as_str() {
                        "Utilities" => data.utilities = val.to_string(),
                        "Health Care" => data.health_care = val.to_string(),
                        "Information Technology" => data.information_technology = val.to_string(),
                        "Industrials" => data.industrials = val.to_string(),
                        "Real Estate" => data.real_estate = val.to_string(),
                        "Consumer Staples" => data.consumer_staples = val.to_string(),
                        "Consumer Discretionary" => data.consumer_discretionary = val.to_string(),
                        "Financials" => data.financials = val.to_string(),
                        "Communication Services" => data.communication_services = val.to_string(),
                        "Materials" => data.materials = val.to_string(),
                        "Energy" => data.energy = val.to_string(),
                        _ => {}
                    }
                }
                final_data.push(data);
            }
            sector.data = final_data;
        }
        Ok(sector)
    }
}

/// Function used to create a [Sector][Sector] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub async fn sector(api_data: (&str, Option<u64>)) -> Result<Sector, String> {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.sector().await
}
