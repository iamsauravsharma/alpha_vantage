use serde_derive::Deserialize;
use std::collections::HashMap;

pub struct Sector {
    information: Option<String>,
    error_message: Option<String>,
    meta_data: Option<MetaData>,
    data: Option<Vec<Data>>,
}

impl Sector {
    fn new() -> Self {
        Self {
            information: None,
            error_message: None,
            meta_data: None,
            data: None,
        }
    }

    pub fn meta_data(&self) -> Option<MetaData> {
        self.meta_data.clone()
    }

    pub fn data(&self) -> Option<Vec<Data>> {
        self.data.clone()
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
        let mut sector = Sector::new();
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
                        "Information Technology" => data.infromation_technology = val.to_string(),
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

#[derive(Deserialize, Clone)]
pub struct MetaData {
    #[serde(rename = "Information")]
    information: String,
    #[serde(rename = "Last Refreshed")]
    last_refreshed: String,
}

impl MetaData {
    pub fn information(&self) -> String {
        self.information.to_string()
    }

    pub fn last_refreshed(&self) -> String {
        self.last_refreshed.to_string()
    }
}

#[derive(Default, Clone)]
pub struct Data {
    rank: String,
    utilites: String,
    health_care: String,
    infromation_technology: String,
    industrials: String,
    real_estate: String,
    consumer_staples: String,
    consumer_discretionary: String,
    financials: String,
    communication_services: String,
    materials: String,
    energy: String,
}
