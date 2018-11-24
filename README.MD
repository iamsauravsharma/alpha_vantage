# ALPHAVANTAGEAPI

**Status:**
[![Travis Build Status][build_badge]][build_link]
[![Dependency status][deps_badge]][deps_link]
[![Code coverage][codecov_badge]][codecov_link]
[![Dependabot Status][dependabot_badge]][dependabot_link]

**Project Info:**
[![License: MIT][license_badge]][license_link]
[![Rust nightly required][rust_badge]][rust_link]
[![LoC][loc_badge]][loc_link]

(WIP) Rust Wrapper/Client built for [Alphavantage][alphavantage_link]  API

### TODO List

[X] StockTimeSeries
[X] Quote Endpoint
[X] Search Endpoint
[X] Exchange Rate
[X] Forex
[ ] Crypto Currency
[ ] Technical Indicators
[ ] Sector Performances

## Add as dependencies
Edit Cargo.toml file to add alphavantageapi as dependencies

```Rust
[dependencies]
alphavantageapi = {git = "https://github.com/iamsauravsharma/alphavantageapi"}
```

## Usage
Sample code to find out exchange rate between two currency(both physical & digital supported)

```Rust
use alphavantageapi::user::*;
use alphavantageapi::exchange::*;

let api_key = alphavantageapi::set_api("YOUR KEY HERE");
let exchange : Exchange  = api_key.exchange("USD","CNY");
let rate : exchange.get_rate().unwrap();
println!("{}",rate);
assert_eq!(rate.is_sign_positive(),true);
```

[build_badge]: https://travis-ci.com/iamsauravsharma/alphavantageapi.svg?branch=master
[build_link]: https://travis-ci.com/iamsauravsharma/alphavantageapi
[deps_badge]: https://deps.rs/repo/github/iamsauravsharma/alphavantageapi/status.svg
[deps_link]: https://deps.rs/repo/github/iamsauravsharma/alphavantageapi
[codecov_badge]: https://img.shields.io/codecov/c/github/iamsauravsharma/alphavantageapi.svg
[codecov_link]: https://codecov.io/gh/iamsauravsharma/alphavantageapi
[dependabot_badge]: https://api.dependabot.com/badges/status?host=github&repo=iamsauravsharma/cargo-cleaner
[dependabot_link]: https://dependabot.com
[license_badge]: https://img.shields.io/github/license/iamsauravsharma/alphavantageapi.svg
[license_link]: LICENSE
[rust_badge]: https://img.shields.io/badge/rust-nightly-blue.svg
[rust_link]: https://rustup.rs
[loc_badge]: https://tokei.rs/b1/github/iamsauravsharma/alphavantageapi
[loc_link]: https://github.com/iamsauravsharma/alphavantageapi
[alphavantage_link]: https://alphavantage.co