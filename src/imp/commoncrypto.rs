// Copyright (c) 2016, 2017 Mark Lee
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

//! A cryptographic hash generator dependent upon OSX's `CommonCrypto`.

use commoncrypto::{hash, hmac};
use commoncrypto::hmac::CCHmacAlgorithm;
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct HMAC {
    context: hmac::HMAC,
}

impl Hasher {
    /// Create a new `Hasher` for the given `Algorithm`.
    pub fn new(algorithm: Algorithm) -> Hasher {
        let cc_algorithm = match algorithm {
            Algorithm::MD5 => hash::CCDigestAlgorithm::kCCDigestMD5,
            Algorithm::SHA1 => hash::CCDigestAlgorithm::kCCDigestSHA1,
            Algorithm::SHA256 => hash::CCDigestAlgorithm::kCCDigestSHA256,
            Algorithm::SHA512 => hash::CCDigestAlgorithm::kCCDigestSHA512,
        };

        Hasher(hash::Hasher::new(cc_algorithm))
    }

    /// Generate a digest from the data written to the `Hasher`.
    pub fn finish(&mut self) -> Vec<u8> {
        let Hasher(ref mut hasher) = *self;
        match hasher.finish() {
            Ok(digest) => digest,
            Err(error) => panic!("CommonCrypto error: {}", error),
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

fn algorithm_to_hmac_type(algorithm: &Algorithm) -> CCHmacAlgorithm {
    match *algorithm {
        Algorithm::MD5 => CCHmacAlgorithm::kCCHmacAlgMD5,
        Algorithm::SHA1 => CCHmacAlgorithm::kCCHmacAlgSHA1,
        Algorithm::SHA256 => CCHmacAlgorithm::kCCHmacAlgSHA256,
        Algorithm::SHA512 => CCHmacAlgorithm::kCCHmacAlgSHA512,
    }
}

impl HMAC {
    /// Create a new `HMAC` for the given `Algorithm` and `key`.
    pub fn new(algorithm: Algorithm, key: &[u8]) -> HMAC {
        HMAC { context: hmac::HMAC::new(algorithm_to_hmac_type(&algorithm), key) }
    }

    /// Generate an HMAC from the key + data written to the `HMAC` instance.
    pub fn finish(&mut self) -> Vec<u8> {
        self.context.finish()
    }
}

impl io::Write for HMAC {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.context.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
