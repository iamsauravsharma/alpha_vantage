#!/usr/bin/env bash/
set -e
if [[ $RUSTFMT_ADDED == "false" ]]
then
  cargo +nightly-"${LAST_AVAILABLE_FMT}" fmt --all -- --check
else
  cargo fmt --all -- --check
fi
if [[ $CLIPPY_ADDED == "false" ]]
then
  cargo +nightly-"${LAST_AVAILABLE_CLIPPY}" clippy --all -- -D warnings
else
    cargo clippy --all -- -D warnings
fi
cargo build --all
cargo doc --no-deps --all
cargo test --all