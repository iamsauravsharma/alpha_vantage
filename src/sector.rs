use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
struct MetaData {
    #[serde(rename = "Information")]
    information: String,
    #[serde(rename = "Last Refreshed")]
    last_refreshed: String,
}

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
    pub fn rank(&self) -> String {
        self.rank.to_string()
    }

    pub fn utilites(&self) -> String {
        self.utilites.to_string()
    }

    pub fn health_care(&self) -> String {
        self.health_care.to_string()
    }

    pub fn information_technology(&self) -> String {
        self.information_technology.to_string()
    }

    pub fn industrials(&self) -> String {
        self.industrials.to_string()
    }

    pub fn real_estate(&self) -> String {
        self.real_estate.to_string()
    }

    pub fn consumer_staples(&self) -> String {
        self.consumer_staples.to_string()
    }

    pub fn consumer_discretionary(&self) -> String {
        self.consumer_discretionary.to_string()
    }

    pub fn financials(&self) -> String {
        self.financials.to_string()
    }

    pub fn communication_services(&self) -> String {
        self.communication_services.to_string()
    }

    pub fn materials(&self) -> String {
        self.materials.to_string()
    }

    pub fn energy(&self) -> String {
        self.energy.to_string()
    }
}

#[derive(Default)]
pub struct Sector {
    error_message: Option<String>,
    information: Option<String>,
    meta_data: Option<MetaData>,
    data: Option<Vec<Data>>,
}

impl Sector {
    pub fn information(&self) -> Result<String, String> {
        self.check_meta_data("information")
    }

    pub fn last_refreshed(&self) -> Result<String, String> {
        self.check_meta_data("last refreshed")
    }

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
