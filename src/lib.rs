#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod exchange;
pub mod forex;
pub mod quote;
pub mod search;
pub mod time_series;
pub mod user;
pub mod util;

use self::user::APIKey;

pub fn set_api(api: &str) -> APIKey {
    APIKey::set_api(api)
}
