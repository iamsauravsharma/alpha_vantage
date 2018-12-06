# ALPHA_VANTAGE

**Status:**
[![Travis Build Status][build_badge]][build_link]
[![Dependency status][deps_badge]][deps_link]
[![Code coverage][codecov_badge]][codecov_link]
[![Dependabot Status][dependabot_badge]][dependabot_link]

**Project Info:**
[![License: MIT][license_badge]][license_link]
[![Rust nightly required][rust_badge]][rust_link]
[![LoC][loc_badge]][loc_link]


**WARNING : may contain bug which may cause undesired functionality** 

(WIP) Rust Wrapper/Client built for [Alphavantage][alphavantage_link]  API

### Project Functionality

- [X] StockTimeSeries
- [X] Quote Endpoint
- [X] Search Endpoint
- [X] Exchange Rate
- [X] Forex
- [ ] Crypto Currency
- [ ] Technical Indicators
- [X] Sector Performances

### TODO LIST AFTER FINISHING PROJECT

- [ ] Optimize code
- [ ] Improve performance of API Call 
- [ ] Use generics and traits
- [ ] Better documentation
- [ ] Publish to [crates.io][cratesio_link]

## Add as dependencies
Edit Cargo.toml file to add alpha_vantage as dependencies

```Rust
[dependencies]
alpha_vantage = {git = "https://github.com/iamsauravsharma/alpha_vantage"}
```

## Usage
Sample code to find out exchange rate between two currency(both physical & digital supported)

```Rust
use alpha_vantage::user::*;
use alpha_vantage::exchange::*;

let api_key = alpha_vantage::set_api("YOUR KEY HERE");
let exchange : Exchange  = api_key.exchange("USD","CNY");
let rate : exchange.get_rate().unwrap();
println!("{}",rate);
assert_eq!(rate.is_sign_positive(),true);
```

[build_badge]: https://travis-ci.com/iamsauravsharma/alpha_vantage.svg?branch=master
[build_link]: https://travis-ci.com/iamsauravsharma/alpha_vantage
[deps_badge]: https://deps.rs/repo/github/iamsauravsharma/alpha_vantage/status.svg
[deps_link]: https://deps.rs/repo/github/iamsauravsharma/alpha_vantage
[codecov_badge]: https://img.shields.io/codecov/c/github/iamsauravsharma/alpha_vantage.svg
[codecov_link]: https://codecov.io/gh/iamsauravsharma/alpha_vantage
[dependabot_badge]: https://api.dependabot.com/badges/status?host=github&repo=iamsauravsharma/cargo-cleaner
[dependabot_link]: https://dependabot.com
[license_badge]: https://img.shields.io/github/license/iamsauravsharma/alpha_vantage.svg
[license_link]: LICENSE
[rust_badge]: https://img.shields.io/badge/rust-nightly-blue.svg
[rust_link]: https://rustup.rs
[loc_badge]: https://tokei.rs/b1/github/iamsauravsharma/alpha_vantage
[loc_link]: https://github.com/iamsauravsharma/alpha_vantage
[alphavantage_link]: https://alphavantage.co
[cratesio_link]: https://crates.io