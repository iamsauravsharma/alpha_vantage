# ALPHA_VANTAGE

**Project status & info:**

| Travis Build Status | Code Coverage | License | Crates Version | Docs |
| :---: | :---: | :---: | :---: | :---: |
| [![Travis Build Status][build_badge]][build_link] | [![Code coverage][codecov_badge]][codecov_link] | [![License: MIT][license_badge]][license_link] | [![Crate][cratesio_badge]][cratesio_link] | [![Docs][docsrs_badge]][docsrs_link] |

**Rust & OS support:**

| Stable | Beta | Nightly | Linux | OSX | Windows |
| :---: | :---: | :---: | :---: | :---: | :---: |
| [![Stable support][stable_supported_badge]][git_link] | [![Beta support][beta_supported_badge]][git_link] | [![Nightly support][nightly_supported_badge]][git_link] | [![Linux support][linux_supported_badge]][git_link] | [![OSX support][osx_supported_badge]][git_link] | [![Windows support][windows_supported_badge]][git_link] |

Rust Client library built for accessing [Alphavantage][alphavantage_link] API.

### Project Functionality

- [X] StockTimeSeries
- [X] Quote Endpoint
- [X] Search Endpoint
- [X] Exchange Rate
- [X] Forex
- [X] Crypto Currency
- [X] Technical Indicators
- [X] Sector Performances

## Add as dependencies
Edit Cargo.toml file to add alpha_vantage as dependencies
```toml
[dependencies]
alpha_vantage = "0.4.0"
```

__OR__
For bleeding edge development use

```toml
[dependencies]
alpha_vantage = {git = "https://github.com/iamsauravsharma/alpha_vantage"}
```

## Usage
Sample code to find out exchange rate between two currency(both physical & digital supported)

```Rust
let api_key = alpha_vantage::set_api("YOUR-API-HERE");
let exchange = api_key.exchange("USD","CNY").await.unwrap();
let rate = exchange.rate();
println!("{}",rate);
```

Similarly using blocking api you can run above example which run same API
```Rust
let api_key = alpha_vantage::blocking::APIkey::set_api("YOUR-API-HERE");
let exchange = api_key.exchange("USD","CNY").unwrap();
let rate = exchange.rate();
println!("{}",rate);
```

[git_link]: https://github.com/iamsauravsharma/alpha_vantage

[build_badge]: https://img.shields.io/travis/com/iamsauravsharma/alpha_vantage.svg?logo=travis
[build_link]: https://travis-ci.com/iamsauravsharma/alpha_vantage

[codecov_badge]: https://img.shields.io/codecov/c/github/iamsauravsharma/alpha_vantage.svg?logo=codecov
[codecov_link]: https://codecov.io/gh/iamsauravsharma/alpha_vantage

[license_badge]: https://img.shields.io/github/license/iamsauravsharma/alpha_vantage.svg
[license_link]: LICENSE

[alphavantage_link]: https://alphavantage.co

[cratesio_badge]: https://img.shields.io/crates/v/alpha_vantage.svg
[cratesio_link]: https://crates.io/crates/alpha_vantage

[docsrs_badge]: https://docs.rs/alpha_vantage/badge.svg
[docsrs_link]: https://docs.rs/alpha_vantage

[stable_supported_badge]: https://img.shields.io/badge/stable-supported-brightgreen.svg?logo=rust
[beta_supported_badge]: https://img.shields.io/badge/beta-supported-brightgreen.svg?logo=rust
[nightly_supported_badge]: https://img.shields.io/badge/nightly-supported-brightgreen.svg?logo=rust
[stable_not_supported_badge]: https://img.shields.io/badge/stable-not%20supported-red.svg?logo=rust
[beta_not_supported_badge]: https://img.shields.io/badge/beta-not%20supported-red.svg?logo=rust
[nightly_not_supported_badge]: https://img.shields.io/badge/nightly-not%20supported-red.svg?logo=rust

[linux_supported_badge]: https://img.shields.io/badge/linux-supported-brightgreen.svg?logo=linux
[osx_supported_badge]: https://img.shields.io/badge/osx-supported-brightgreen.svg?logo=apple
[windows_supported_badge]: https://img.shields.io/badge/windows-supported-brightgreen.svg?logo=windows
[linux_not_supported_badge]: https://img.shields.io/badge/linux-not%20supported-red.svg?logo=linux
[osx_not_supported_badge]: https://img.shields.io/badge/osx-not%20supported-red.svg?logo=apple
[windows_not_supported_badge]: https://img.shields.io/badge/windows-not%20supported-red.svg?logo=windows
