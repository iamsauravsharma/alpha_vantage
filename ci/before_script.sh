#!/usr/bin/env bash/
if [[ "$RUSTFMT_ADDED" == "false" ]]; then
  LAST_AVAILABLE=$(curl https://rust-lang.github.io/rustup-components-history/x86_64-apple-darwin/rustfmt)
  rustup toolchain install nightly-${LAST_AVAILABLE}
  rustup default nightly-${LAST_AVAILABLE}
  rustup component add rustfmt
fi
if [[ "$CLIPPY_ADDED" == "true" ]]; then
  LAST_AVAILABLE=$(curl https://rust-lang.github.io/rustup-components-history/x86_64-apple-darwin/clippy)
  rustup toolchain install nightly-${LAST_AVAILABLE}
  rustup default nightly-${LAST_AVAILABLE}
  rustup component add clippy
fi
rustup -V
rustc -V
cargo -V
rustfmt -V
clippy -V