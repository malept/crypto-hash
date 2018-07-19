#!/bin/bash -xe

cargo test --target $TARGET

if test "$TRAVIS_OS_NAME" = "linux" -a "$TARGET" = "x86_64-unknown-linux-gnu"; then
    if test "$TRAVIS_RUST_VERSION" = "stable"; then
        cargo doc
        check_param="--write-mode=diff"
    else
        check_param="--check"
    fi

    if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
        cargo fmt -- $check_param $(git diff --name-only "$TRAVIS_COMMIT" "$TRAVIS_BRANCH" | grep \.rs$)
    else
        cargo fmt -- $check_param $(git show --format= --name-only "$TRAVIS_COMMIT_RANGE" | sort -u | grep \.rs$)
    fi
fi

if test "$TRAVIS_RUST_VERSION" = "nightly"; then
    cargo clippy --target $TARGET -- --allow clippy_pedantic
fi
