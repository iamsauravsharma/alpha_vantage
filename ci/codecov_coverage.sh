#!/usr/bin/env bash
set -ex

bash <(curl https://raw.githubusercontent.com/xd009642/tarpaulin/master/travis-install.sh)
cargo tarpaulin --all-features --verbose --run-types Tests Doctests --out Xml
bash <(curl -s https://codecov.io/bash)

set +x
