#!/bin/bash -e

# this will (should) run the same things as ./.github/workflows/rust.yml,
# to allow you to check them locally, first.

set -e

# cannot install `cargo-outdated` currently
cargo install --locked cargo-deny cargo-udeps cargo-audit cargo-pants || true # cargo-outdated

cargo deny check --config config/cargo-deny.toml

# unable to install currently
# cargo outdated --exit-code 1

# TODO: get GH action working with nightly toolchain
# cargo +nightly udeps

rm -rf ~/.cargo/advisory-db
cargo audit
cargo pants
cargo check

echo Passed Check!
