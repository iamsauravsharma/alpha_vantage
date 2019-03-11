# ALPHA_VANTAGE

**Status:**

| Travis Build Status | Dependency Status | Code Coverage |
| :---: | :---: | :---: |
| [![Travis Build Status][build_badge]][build_link] | [![Dependency status][deps_badge]][deps_link] | [![Code coverage][codecov_badge]][codecov_link] |

**Project Info:**

| License | LoC | Crates Version |
| :---: | :---: | :---: |
| [![License: MIT][license_badge]][license_link] | [![LoC][loc_badge]][loc_link] | [![Crate][cratesio_badge]][cratesio_link] |

**Rust version support:**

| Stable | Beta | Nightly |
| :---: | :---: | :---: |
| [![Stable support][stable_supported_badge]][git_link] | [![Beta support][beta_supported_badge]][git_link] | [![Nightly support][nightly_supported_badge]][git_link] |

**OS support:**

| Linux | OSX | Windows |
| :---: | :---: | :---: |
| [![Linux support][linux_supported_badge]][git_link] | [![OSX support][osx_supported_badge]][git_link] | [![Windows support][windows_supported_badge]][git_link] |

**WARNING : may contain bug which may cause undesired functionality** 

Rust Wrapper/Client built for [Alphavantage][alphavantage_link]  API.
Project is completed  and core functionality is already implemented out totally and project is in usable state.
Better documentation and optimization are remaining before publishing to [crates.io][cratesio_link]

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
```
[dependencies]
alpha_vantage = 0.1.0
```

__OR__
For bleeding edge development use

```
[dependencies]
alpha_vantage = {git = "https://github.com/iamsauravsharma/alpha_vantage"}
```

## Usage
Sample code to find out exchange rate between two currency(both physical & digital supported)

```Rust
let api_key = alpha_vantage::set_api("YOUR-API-HERE");
let exchange = api_key.exchange("USD","CNY");
let rate = exchange.rate().unwrap();
println!("{}",rate);
```

[git_link]: https://github.com/iamsauravsharma/alpha_vantage

[build_badge]: https://img.shields.io/travis/com/iamsauravsharma/alpha_vantage.svg?logo=travis
[build_link]: https://travis-ci.com/iamsauravsharma/alpha_vantage

[deps_badge]: https://deps.rs/repo/github/iamsauravsharma/alpha_vantage/status.svg
[deps_link]: https://deps.rs/repo/github/iamsauravsharma/alpha_vantage

[codecov_badge]: https://img.shields.io/codecov/c/github/iamsauravsharma/alpha_vantage.svg?logo=codecov
[codecov_link]: https://codecov.io/gh/iamsauravsharma/alpha_vantage

[license_badge]: https://img.shields.io/github/license/iamsauravsharma/alpha_vantage.svg
[license_link]: LICENSE

[loc_badge]: https://tokei.rs/b1/github/iamsauravsharma/alpha_vantage
[loc_link]: https://github.com/iamsauravsharma/alpha_vantage

[alphavantage_link]: https://alphavantage.co

[cratesio_badge]: https://img.shields.io/crates/v/alpha_vantage.svg
[cratesio_link]: https://crates.io/crates/alpha_vantage

[stable_supported_badge]: https://img.shields.io/badge/stable-supported-brightgreen.svg?logo=rust
[beta_supported_badge]: https://img.shields.io/badge/beta-supported-brightgreen.svg?logo=rust
[nightly_supported_badge]: https://img.shields.io/badge/nightly-supported-brightgreen.svg?logo=rust
[stable_not_supported_badge]: https://img.shields.io/badge/stable-not%20supported-red.svg?logo=rust
[beta_not_supported_badge]: https://img.shields.io/badge/beta-not%20supported-red.svg?logo=rust
[nighlty_not_supported_badge]: https://img.shields.io/badge/nightly-not%20supported-red.svg?logo=rust

[linux_supported_badge]: https://img.shields.io/badge/linux-supported-brightgreen.svg?logo=linux
[osx_supported_badge]: https://img.shields.io/badge/osx-supported-brightgreen.svg?logo=apple
[windows_supported_badge]: https://img.shields.io/badge/windows-supported-brightgreen.svg?logo=windows
[linux_not_supported_badge]: https://img.shields.io/badge/linux-not%20supported-red.svg?logo=linux
[osx_not_supported_badge]: https://img.shields.io/badge/osx-not%20supported-red.svg?logo=apple
[windows_not_supported_badge]: https://img.shields.io/badge/windows-not%20supported-red.svg?logo=windows
