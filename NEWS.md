# Changes by Version

## Unreleased

### Added

* SHA-1 algorithm

## [0.1.0] - 2016-06-26

This release signifies the minimum amount of algorithms and implementations necessary for
[HTTP digest authentication](https://tools.ietf.org/html/rfc7616).

### Added

Algorithms:

* MD5
* SHA256
* SHA512

Implementations:

* CommonCrypto (OS X)
* CryptoAPI (Windows)
* OpenSSL (Linux/BSD/etc.)

[0.1.0]: https://github.com/malept/crypto-hash/releases/tag/v0.1.0
