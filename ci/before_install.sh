#!/bin/bash -xe

# Mostly from https://github.com/japaric/rust-everywhere
# Load the correct rust if it's not already there

case $TARGET in
  # Install standard libraries needed for cross compilation
  arm-unknown-linux-gnueabihf | \
  i686-apple-darwin | \
  i686-unknown-linux-gnu | \
  x86_64-unknown-linux-musl)
    rustup target add --toolchain $TRAVIS_RUST_VERSION $TARGET
    ;;
  # Nothing to do for native builds
  *) ;;
esac
