#!/bin/bash

set -e

# load travis-cargo
pip install 'travis-cargo<0.2' --user

test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update

if test "$TRAVIS_RUST_VERSION" = "nightly"; then
    test -x $HOME/.cargo/bin/cargo-clippy || cargo install clippy
    cargo install-update clippy
fi
