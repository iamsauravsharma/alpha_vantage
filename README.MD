# ALPHA_VANTAGE

**Project status & info:**

| License | Crates Version | Docs |
| :---: | :---: | :---: |
| [![License: MIT][license_badge]][license_link] | [![Crate][cratesio_badge]][cratesio_link] | [![Docs][docsrs_badge]][docsrs_link] |

Rust Client library built for accessing [Alphavantage][alphavantage_link] API.

### Project Functionality

- [X] Crypto Currency
- [X] Custom function
- [X] Earning
- [X] Economic Indicator
- [X] Exchange Rate
- [X] Forex
- [X] Quote Endpoint
- [X] Search Endpoint
- [X] Sector Performances
- [X] Stock Time Series
- [X] Technical Indicators

## Add as dependencies
Edit Cargo.toml file to add alpha_vantage as dependencies
```toml
[dependencies]
alpha_vantage = "0.8.0"
```

__OR__
For bleeding edge development use

```toml
[dependencies]
alpha_vantage = {git = "https://github.com/iamsauravsharma/alpha_vantage"}
```

## Usage
Sample code to find exchange rate between two currency(both physical & digital supported)

```rust
let api_key = alpha_vantage::set_api("YOUR-API-HERE", reqwest::Client::new());
let exchange = api_key.exchange("USD","CNY").json().await.unwrap();
let rate = exchange.rate();
println!("{}",rate);
```

See docs for detailed example of other functionality

[license_badge]: https://img.shields.io/github/license/iamsauravsharma/alpha_vantage.svg?style=for-the-badge
[license_link]: LICENSE

[alphavantage_link]: https://www.alphavantage.co

[cratesio_badge]: https://img.shields.io/crates/v/alpha_vantage.svg?style=for-the-badge
[cratesio_link]: https://crates.io/crates/alpha_vantage

[docsrs_badge]: https://img.shields.io/docsrs/alpha_vantage/latest?style=for-the-badge
[docsrs_link]: https://docs.rs/alpha_vantage
