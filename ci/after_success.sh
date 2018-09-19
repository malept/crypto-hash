#!/bin/bash -e

if test "$TARGET" = "all-style-docs"; then
    # upload the documentation from the build with stable (automatically only actually
    # runs on the master branch, not individual PRs)
    cargo doc-upload
elif test "$TARGET" = "x86_64-unknown-linux-gnu" -a "$TRAVIS_RUST_VERSION" = "stable"; then
    # measure code coverage and upload to coveralls.io (the verify
    # argument mitigates kcov crashes due to malformed debuginfo, at the
    # cost of some speed <https://github.com/huonw/travis-cargo/issues/12>)
    cargo coveralls --exclude-pattern=/test.rs,*.c,openssl-sys
fi
