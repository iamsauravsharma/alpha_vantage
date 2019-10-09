#!/usr/bin/env bash/
set -e

run_all_cargo_command(){
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
}

# if $TARGET is present then run only build over that target else run_all_cargo_command
if [[ -n $TARGET ]]
then
    cargo build --all --target="$TARGET"
else
  run_all_cargo_command
fi