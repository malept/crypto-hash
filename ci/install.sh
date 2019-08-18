#!/bin/bash -xe

install_cargo_plugins() {
    cargo install cargo-update || echo "cargo-update already installed"
    cargo install cargo-travis || echo "cargo-travis already installed"
    cargo install cargo-tarpaulin || echo "cargo-tarpaulin already installed"
    cargo install-update -a
}

install_style_docs_dependencies() {
    rustup component add rustfmt clippy --toolchain=$TRAVIS_RUST_VERSION
    install_cargo_plugins
}

install_compile_dependencies() {
    # Builds for iOS are done on OSX, but require the specific target to be installed.
    case $TARGET in
        *-apple-ios)
            rustup target install $TARGET
            ;;
    esac

    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from homebrew's coreutils.
    fi

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target

    if test "$TARGET" = "x86_64-unknown-linux-gnu" -a "$TRAVIS_RUST_VERSION" = "stable"; then
        install_cargo_travis
    fi
}

main() {
    if test "$TARGET" = "all-style-docs"; then
        install_style_docs_dependencies
    else
        install_compile_dependencies
    fi
}

main
