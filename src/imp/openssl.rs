// Copyright (c) 2015, 2016 Mark Lee
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! A cryptographic hash digest generator dependent upon `OpenSSL`.

#![warn(missing_docs)]

use openssl::hash;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use std::io;
use super::Algorithm;

/// Generator of digests using a cryptographic hash function.
///
/// # Examples
///
/// ```rust
/// use crypto_hash::{Algorithm, Hasher};
/// use std::io::Write;
///
/// let mut hasher = Hasher::new(Algorithm::SHA256);
/// hasher.write_all(b"crypto");
/// hasher.write_all(b"-");
/// hasher.write_all(b"hash");
/// let result = hasher.finish();
/// let expected =
///     b"\xfd\x1a\xfb`\"\xcdMG\xc8\x90\x96\x1cS9(\xea\xcf\xe8!\x9f\x1b%$\xf7\xfb*a\x84}\xdf\x8c'"
///     .to_vec();
/// assert_eq!(expected, result)
/// ```
pub struct Hasher(hash::Hasher);

/// Generator of Hash-based Message Authentication Codes (HMACs).
///
/// # Examples
///
/// ```rust
/// use crypto_hash::{Algorithm, HMAC};
/// use std::io::Write;
///
/// let mut hmac = HMAC::new(Algorithm::SHA256, b"");
/// hmac.write_all(b"crypto");
/// hmac.write_all(b"-");
/// hmac.write_all(b"hash");
/// let result = hmac.finish();
/// let expected =
///     b"\x8e\xd6\xcd0\xba\xc2\x9e\xdc\x0f\xcc3\x07\xd4D\xdb6\xa6\xe8/\xf3\x94\xe6\xac\xa2\x01l\x03/*1\x1f$"
///     .to_vec();
/// assert_eq!(expected, result)
/// ```
pub struct HMAC(hmac::HMAC);

fn algorithm_to_hash_type(algorithm: Algorithm) -> hash::Type {
    match algorithm {
        Algorithm::MD5 => hash::Type::MD5,
        Algorithm::SHA1 => hash::Type::SHA1,
        Algorithm::SHA256 => hash::Type::SHA256,
        Algorithm::SHA512 => hash::Type::SHA512,
    }
}

impl Hasher {
    /// Create a new `Hasher` for the given `Algorithm`.
    pub fn new(algorithm: Algorithm) -> Hasher {
        match hash::Hasher::new(algorithm_to_hash_type(algorithm)) {
            Ok(hasher) => Hasher(hasher),
            Err(error_stack) => panic!("OpenSSL error(s): {}", error_stack)
        }
    }

    /// Generate a digest from the data written to the `Hasher`.
    pub fn finish(&mut self) -> Vec<u8> {
        let Hasher(ref mut hasher) = *self;
        match hasher.finish() {
            Ok(digest) => digest,
            Err(error_stack) => panic!("OpenSSL error(s): {}", error_stack)
        }
    }
}

impl io::Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let Hasher(ref mut hasher) = *self;
        hasher.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let Hasher(ref mut hasher) = *self;
        hasher.flush()
    }
}

impl HMAC {
    /// Create a new `HMAC` for the given `Algorithm` and `key`.
    pub fn new(algorithm: Algorithm, key: &[u8]) -> HMAC {
        match hmac::HMAC::new(algorithm_to_hash_type(algorithm), key) {
            Ok(hmac) => HMAC(hmac),
            Err(error_stack) => panic!("OpenSSL error(s): {}", error_stack)
        }
    }

    /// Generate an HMAC from the key + data written to the `HMAC` instance.
    pub fn finish(&mut self) -> Vec<u8> {
        let HMAC(ref mut hmac) = *self;
        match hmac.finish() {
            Ok(data) => data,
            Err(error_stack) => panic!("OpenSSL error(s): {}", error_stack)
        }
    }
}

impl io::Write for HMAC {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let HMAC(ref mut hmac) = *self;
        hmac.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let HMAC(ref mut hmac) = *self;
        hmac.flush()
    }
}
