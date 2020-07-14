#!/bin/bash -e

if test "$TARGET" = "all-style-docs"; then
    # upload the documentation from the build with stable (automatically only actually
    # runs on the main branch, not individual PRs)
    cargo doc-upload
elif test "$TARGET" = "x86_64-unknown-linux-gnu" -a "$TRAVIS_RUST_VERSION" = "stable"; then
    cargo tarpaulin --out Xml
    bash <(curl -s https://codecov.io/bash)
fi
