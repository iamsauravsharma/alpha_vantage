#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde_json;

pub mod user;

use self::user::APIKey;

pub fn set_api(api: &str) -> APIKey {
    APIKey::set_api(api)
}
