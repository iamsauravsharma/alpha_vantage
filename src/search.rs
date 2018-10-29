/// struct for storing search method data
#[derive(Debug, Deserialize)]
pub struct Search {
    #[serde(rename = "bestMatches")]
    matches: Option<Vec<DataValue>>,
}

// Struct which stores matches data for search keyword
#[derive(Debug, Clone, Deserialize)]
struct DataValue {
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

/// struct used for returning search result value
#[derive(Debug, Default)]
pub struct PublicDataValue {
    pub symbol: String,
    pub name: String,
    pub data_type: String,
    pub region: String,
    pub market_open: String,
    pub market_close: String,
    pub time_zone: String,
    pub currency: String,
    pub match_score: String,
}

impl Search {
    // Return result of search
    pub fn result(&self) -> Vec<PublicDataValue> {
        let mut vec = Vec::new();
        if let Some(value) = self.matches.clone() {
            for data in value.iter() {
                let data = data.clone();
                let mut value: PublicDataValue = Default::default();
                value.symbol = data.symbol;
                value.name = data.name;
                value.data_type = data.data_type;
                value.region = data.region;
                value.market_open = data.market_open;
                value.market_close = data.market_close;
                value.time_zone = data.time_zone;
                value.currency = data.currency;
                value.match_score = data.match_score;
                vec.push(value);
            }
        }
        vec
    }
}
