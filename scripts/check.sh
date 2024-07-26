#!/bin/bash -e

# this will (should) run the same things as ./.github/workflows/rust.yml,
# to allow you to check them locally, first.

set -e

cargo install --locked cargo-deny cargo-udeps cargo-audit cargo-pants
# cannot install `cargo-outdated` currently

cargo deny check --config config/cargo-deny.toml

# unable to install currently
# cargo outdated --exit-code 1

# requires nightly?
cargo +nightly udeps

rm -rf ~/.cargo/advisory-db
cargo audit
cargo pants
cargo check
echo Passed Check!
