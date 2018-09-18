#!/bin/bash -xe

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    if test "$TARGET" = "x86_64-unknown-linux-gnu" -a "$TRAVIS_RUST_VERSION" = "stable"; then
        cargo doc

        if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
            cargo fmt -- --check $(git diff --name-only "$TRAVIS_COMMIT" "$TRAVIS_BRANCH" | grep \.rs$)
        else
            cargo fmt -- --check $(git show --format= --name-only "$TRAVIS_COMMIT_RANGE" | sort -u | grep \.rs$)
        fi

        cargo clippy --target $TARGET -- --allow clippy_pedantic
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
