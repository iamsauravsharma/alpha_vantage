#!/usr/bin/env bash/
cargo fmt --all -- --check
cargo clippy --all -- -D warnings
cargo build --all
cargo doc --no-deps --all
cargo test --all