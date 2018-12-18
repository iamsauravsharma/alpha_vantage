//! # Example
//! ```
//! fn sector() {
//!     let api = alpha_vantage::set_api("demo");
//!     let sector = api.sector();
//!     assert_eq!(sector.information().is_ok(), true);
//! }
//! ```

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
    pub fn rank(&self) -> String {
        self.rank.to_string()
    }

    /// Return utilites score
    pub fn utilites(&self) -> String {
        self.utilites.to_string()
    }

    /// Return health care score
    pub fn health_care(&self) -> String {
        self.health_care.to_string()
    }

    /// Return out information technology
    pub fn information_technology(&self) -> String {
        self.information_technology.to_string()
    }

    /// Return industrials scores
    pub fn industrials(&self) -> String {
        self.industrials.to_string()
    }

    /// Return out real estate value
    pub fn real_estate(&self) -> String {
        self.real_estate.to_string()
    }

    /// Return consumer staples value
    pub fn consumer_staples(&self) -> String {
        self.consumer_staples.to_string()
    }

    /// Return out value for consumer discretionary
    pub fn consumer_discretionary(&self) -> String {
        self.consumer_discretionary.to_string()
    }

    /// Return out for financials
    pub fn financials(&self) -> String {
        self.financials.to_string()
    }

    /// Gives value of communication services
    pub fn communication_services(&self) -> String {
        self.communication_services.to_string()
    }

    /// Gives materials value
    pub fn materials(&self) -> String {
        self.materials.to_string()
    }

    /// Gives out energy data
    pub fn energy(&self) -> String {
        self.energy.to_string()
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
    pub fn information(&self) -> Result<String, String> {
        self.check_meta_data("information")
    }

    /// Return last refreshed time
    pub fn last_refreshed(&self) -> Result<String, String> {
        self.check_meta_data("last refreshed")
    }

    /// Return vector of data in Result
    pub fn data(&self) -> Result<Vec<Data>, String> {
        if let Some(data) = &self.data {
            Ok(data.to_vec())
        } else if let Some(error) = &self.error_message {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    /// Check a meta data is present or not
    fn check_meta_data(&self, name: &str) -> Result<String, String> {
        if let Some(meta_data) = &self.meta_data {
            let value = match name {
                "information" => &meta_data.information,
                "last refreshed" => &meta_data.last_refreshed,
                _ => "",
            };
            Ok(value.to_string())
        } else if let Some(error) = &self.error_message {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
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
            for (key, val) in temp_data.iter() {
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
