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
    else
        target=x86_64-apple-darwin
    fi

    # Pinning to v0.1.16 because v0.2 removes openssl from the images
    # https://github.com/rust-embedded/cross/pull/322
    local tag=v0.1.16
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-embedded/cross \
           --tag $tag \
           --target $target

    if test "$TARGET" = "x86_64-unknown-linux-gnu" -a "$TRAVIS_RUST_VERSION" = "stable"; then
        install_cargo_plugins
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
