#!/usr/bin/env bash
rustup component add rustfmt && export RUSTFMT_ADDED="true" || export RUSTFMT_ADDED="false"
rustup component add clippy && export CLIPPY_ADDED="true" || export CLIPPY_ADDED="false"