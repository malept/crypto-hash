#!/bin/bash

set -e
if test "$TARGET" = "x86_64-unknown-linux-gnu"; then
    # upload the documentation from the build with stable (automatically only actually
    # runs on the master branch, not individual PRs)
    travis-cargo --only stable doc-upload
    # measure code coverage and upload to coveralls.io (the verify
    # argument mitigates kcov crashes due to malformed debuginfo, at the
    # cost of some speed <https://github.com/huonw/travis-cargo/issues/12>)
    travis-cargo --only stable coveralls --no-sudo --verify --exclude-pattern=/test.rs,*.c,openssl-sys
fi
