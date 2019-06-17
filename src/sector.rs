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
use serde_derive::Deserialize;
use std::collections::HashMap;

/// Stores Metadata
#[derive(Deserialize, Clone)]
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
    utilites: String,
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
    pub fn rank(&self) -> &str {
        &self.rank
    }

    /// Return utilites score
    pub fn utilites(&self) -> &str {
        &self.utilites
    }

    /// Return health care score
    pub fn health_care(&self) -> &str {
        &self.health_care
    }

    /// Return out information technology
    pub fn information_technology(&self) -> &str {
        &self.information_technology
    }

    /// Return industrials scores
    pub fn industrials(&self) -> &str {
        &self.industrials
    }

    /// Return out real estate value
    pub fn real_estate(&self) -> &str {
        &self.real_estate
    }

    /// Return consumer staples value
    pub fn consumer_staples(&self) -> &str {
        &self.consumer_staples
    }

    /// Return out value for consumer discretionary
    pub fn consumer_discretionary(&self) -> &str {
        &self.consumer_discretionary
    }

    /// Return out for financials
    pub fn financials(&self) -> &str {
        &self.financials
    }

    /// Gives value of communication services
    pub fn communication_services(&self) -> &str {
        &self.communication_services
    }

    /// Gives materials value
    pub fn materials(&self) -> &str {
        &self.materials
    }

    /// Gives out energy data
    pub fn energy(&self) -> &str {
        &self.energy
    }
}

/// Stores sector data
#[derive(Default)]
pub struct Sector {
    error_message: Option<String>,
    information: Option<String>,
    meta_data: Option<MetaData>,
    data: Option<Vec<Data>>,
}

impl Sector {
    /// Return sector information
    ///
    /// ```
    /// let api = alpha_vantage::set_api("demo");
    /// let sector = api.sector();
    /// let information = sector.information();
    /// assert_eq!(
    ///     information.unwrap(),
    ///     "US Sector Performance (realtime & historical)"
    /// );
    /// ```
    pub fn information(&self) -> Result<&str, &str> {
        self.check_meta_data("information")
    }

    /// Return last refreshed time
    pub fn last_refreshed(&self) -> Result<&str, &str> {
        self.check_meta_data("last refreshed")
    }

    /// Return vector of data in Result
    pub fn data(&self) -> Result<Vec<Data>, &str> {
        if let Some(data) = &self.data {
            Ok(data.to_vec())
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
        }
    }

    /// Check a meta data is present or not
    fn check_meta_data(&self, name: &str) -> Result<&str, &str> {
        if let Some(meta_data) = &self.meta_data {
            let value = match name {
                "information" => &meta_data.information,
                "last refreshed" => &meta_data.last_refreshed,
                _ => "",
            };
            Ok(value)
        } else if let Some(error) = &self.error_message {
            Err(error)
        } else if let Some(information) = &self.information {
            Err(information)
        } else {
            Err("Unknown error")
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
    /// Convert out sectorhelper to sector
    pub(crate) fn convert(self) -> Sector {
        let mut sector = Sector::default();
        sector.information = self.information;
        sector.error_message = self.error_message;
        sector.meta_data = self.meta_data;
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
                        "Utilities" => data.utilites = val.to_string(),
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
            sector.data = Some(final_data);
        }
        sector
    }
}

/// Function used to create a [Sector][Sector] struct.
///
/// Instead of using this function directly calling through [APIKey][APIKey]
/// method is recommended
pub fn sector(api_data: (&str, Option<u64>)) -> Sector {
    let api;
    if let Some(timeout) = api_data.1 {
        api = APIKey::set_with_timeout(api_data.0, timeout);
    } else {
        api = APIKey::set_api(api_data.0);
    }
    api.sector()
}
