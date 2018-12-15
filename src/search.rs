use serde_derive::Deserialize;

/// struct for storing search method data
#[derive(Debug, Deserialize)]
pub struct Search {
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "bestMatches")]
    matches: Option<Vec<DataValue>>,
}

// Struct which stores matches data for search keyword
#[derive(Debug, Clone, Deserialize)]
pub struct DataValue {
    #[serde(rename = "1. symbol")]
    symbol: String,
    #[serde(rename = "2. name")]
    name: String,
    #[serde(rename = "3. type")]
    data_type: String,
    #[serde(rename = "4. region")]
    region: String,
    #[serde(rename = "5. marketOpen")]
    market_open: String,
    #[serde(rename = "6. marketClose")]
    market_close: String,
    #[serde(rename = "7. timezone")]
    time_zone: String,
    #[serde(rename = "8. currency")]
    currency: String,
    #[serde(rename = "9. matchScore")]
    match_score: String,
}

impl DataValue {
    pub fn symbol(&self) -> String {
        self.symbol.to_string()
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn data_type(&self) -> String {
        self.data_type.to_string()
    }

    pub fn region(&self) -> String {
        self.region.to_string()
    }

    pub fn market_open(&self) -> String {
        self.market_open.to_string()
    }

    pub fn market_close(&self) -> String {
        self.market_close.to_string()
    }

    pub fn time_zone(&self) -> String {
        self.time_zone.to_string()
    }

    pub fn currency(&self) -> String {
        self.currency.to_string()
    }

    pub fn match_score(&self) -> f64 {
        self.match_score.trim().parse::<f64>().unwrap()
    }
}

impl Search {
    // Return result of search
    pub fn result(&self) -> Result<Vec<DataValue>, String> {
        if let Some(entry) = &self.matches {
            Ok(entry.to_vec())
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }
}
