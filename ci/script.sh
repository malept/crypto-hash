#!/bin/bash -xe

build_and_test() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ -n $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release
}

style_and_docs() {
    cargo doc

    if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
        cargo fmt -- --check $(git diff --name-only "$TRAVIS_COMMIT" "$TRAVIS_BRANCH" | grep \.rs$)
    else
        cargo fmt -- --check $(git show --format= --name-only "$TRAVIS_COMMIT_RANGE" | sort -u | grep \.rs$)
    fi

    cargo clippy -- --allow clippy::pedantic
}

main() {
    if test "$TARGET" = "all-style-docs"; then
        style_and_docs
    else
        build_and_test
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
