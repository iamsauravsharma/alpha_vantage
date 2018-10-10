pub struct APIKey(String);

pub mod exchange;

impl APIKey {
    pub fn set_api(api: &str) -> APIKey {
        APIKey(api.to_string())
    }

    pub fn get_api(&self) -> String {
        self.0.clone()
    }

    pub fn exchange(&self, from_currency: &str, to_currency: &str) -> f64 {
        exchange::Exchange::new(
            from_currency.to_string(),
            to_currency.to_string(),
            self.0.clone(),
        );
        0.0
    }
}
