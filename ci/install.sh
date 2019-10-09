#!/usr/bin/env bash
set -e

# if $TARGET env is present install cross else install rustfmt and clippy
if [[ -n $TARGET ]]
then
    rustup target add "$TARGET"
else
  # add rustfmt and clippy to target
  rustup component add rustfmt && export RUSTFMT_ADDED="true" || export RUSTFMT_ADDED="false"
  rustup component add clippy && export CLIPPY_ADDED="true" || export CLIPPY_ADDED="false"
fi