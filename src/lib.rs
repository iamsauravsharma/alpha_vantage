#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde_json;

pub mod exchange;
pub mod quote;
pub mod time_series;
pub mod user;
pub mod util;
pub mod search;
pub mod forex;

use self::user::APIKey;

pub fn set_api(api: &str) -> APIKey {
    APIKey::set_api(api)
}
