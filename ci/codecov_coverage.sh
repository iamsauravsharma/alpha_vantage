#!/usr/bin/env bash
set -ex

cargo install cargo-tarpaulin
cargo tarpaulin --all-features --verbose --run-types Tests Doctests --out Xml
bash <(curl -s https://codecov.io/bash)

set +x
