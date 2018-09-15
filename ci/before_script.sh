#!/bin/bash -e

if test "$TRAVIS_OS_NAME" = "linux" -a "$TARGET" = "x86_64-unknown-linux-gnu" -a "$TRAVIS_RUST_VERSION" = "stable"; then
    rustup component add rustfmt-preview clippy-preview --toolchain=$TRAVIS_RUST_VERSION
fi

cargo install cargo-update || echo "cargo-update already installed"
cargo install cargo-travis || echo "cargo-travis already installed"
cargo install-update -a
