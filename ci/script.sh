#!/bin/bash

# the main build
set -e

travis-cargo build -- --target $TARGET
travis-cargo test -- --target $TARGET

if test "$TRAVIS_OS_NAME" = "linux" -a "$TARGET" = "x86_64-unknown-linux-gnu"; then
    travis-cargo --only stable doc
fi

if test "$TRAVIS_RUST_VERSION" = "nightly"; then
    cargo clippy --target $TARGET --features=$TRAVIS_CARGO_NIGHTLY_FEATURE -- -Wclippy_pedantic
fi
