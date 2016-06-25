# `crypto-hash`

[![Linux/OS X Status](https://travis-ci.org/malept/crypto-hash.svg?branch=master)](https://travis-ci.org/malept/crypto-hash)
[![Windows status](https://ci.appveyor.com/api/projects/status/xwc9nb4633b5n67r?svg=true)](https://ci.appveyor.com/project/malept/crypto-hash)

`crypto-hash` is a Rust wrapper around OS-level implementations of cryptographic hash functions.

The purpose of this crate is to provide access to hash algorithms with as few dependencies as
possible. This means that when possible, the library uses the hashing functions that are provided by
the given operating system's bundled cryptographic libraries.

## Supported Implementations

By operating system:

* Windows: CryptoAPI
* OS X: CommonCrypto
* Linux/BSD/etc.: OpenSSL

## Supported Algorithms

* MD5
* SHA256
* SHA512

## Usage

Add `crypto-hash` to your project's `Cargo.toml`. For more details, consult the
[Cargo guide](http://doc.crates.io/guide.html#adding-dependencies).

## Acknowledgements

This crate was inspired by [rust-native-tls](https://github.com/sfackler/rust-native-tls) and
[crypto-bench](https://github.com/briansmith/crypto-bench).

## Legal

`crypto-hash` is copyrighted under the terms of the MIT license. See LICENSE for details.
