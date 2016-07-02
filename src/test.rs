// Copyright (c) 2016 Mark Lee
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

#![cfg(test)]

use hex::ToHex;
use std::io::Write;
use super::{Algorithm, Hasher, hex_digest, hex_hmac, HMAC};

// From Wikipedia
const HMAC_MD5_EMPTY_STRING: &'static str = "74e6f7298a9c2d168935f58c001bad88";
const HMAC_SHA1_EMPTY_STRING: &'static str = "fbdb1d1b18aa6c08324b7d64b71fb76370690e1d";
const HMAC_SHA256_EMPTY_STRING: &'static str = "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad";
const MD5_EMPTY_STRING: &'static str = "d41d8cd98f00b204e9800998ecf8427e";
const SHA1_EMPTY_STRING: &'static str = "da39a3ee5e6b4b0d3255bfef95601890afd80709";
const SHA256_EMPTY_STRING: &'static str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
const SHA512_EMPTY_STRING: &'static str = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
const TO_HASH: &'static str = "The quick brown fox jumps over the lazy dog";
const TO_HASH_HMAC_MD5: &'static str = "80070713463e7749b90c2dc24911e275";
const TO_HASH_MD5: &'static str = "9e107d9d372bb6826bd81d3542a419d6";

#[test]
fn hmac_md5_empty_string() {
    assert_hex_hmac_empty_string(Algorithm::MD5, HMAC_MD5_EMPTY_STRING)
}

#[test]
fn hmac_sha1_empty_string() {
    assert_hex_hmac_empty_string(Algorithm::SHA1, HMAC_SHA1_EMPTY_STRING)
}

#[test]
fn hmac_sha256_empty_string() {
    assert_hex_hmac_empty_string(Algorithm::SHA256, HMAC_SHA256_EMPTY_STRING)
}

#[test]
fn md5_empty_string() {
    assert_hex_hashed_empty_string(Algorithm::MD5, MD5_EMPTY_STRING)
}

#[test]
fn sha1_empty_string() {
    assert_hex_hashed_empty_string(Algorithm::SHA1, SHA1_EMPTY_STRING)
}

#[test]
fn sha256_empty_string() {
    assert_hex_hashed_empty_string(Algorithm::SHA256, SHA256_EMPTY_STRING)
}

#[test]
fn sha512_empty_string() {
    assert_hex_hashed_empty_string(Algorithm::SHA512, SHA512_EMPTY_STRING)
}

#[test]
fn hasher_sans_write() {
    let mut hasher = Hasher::new(Algorithm::MD5);
    let actual = hasher.finish().to_hex();
    assert_eq!(MD5_EMPTY_STRING, actual)
}

#[test]
fn hasher_with_write() {
    let mut hasher = Hasher::new(Algorithm::MD5);
    hasher.write_all(TO_HASH.as_bytes()).expect("Could not write to hasher");
    let actual = hasher.finish().to_hex();
    assert_eq!(TO_HASH_MD5, actual)
}

#[test]
fn hmac_sans_write() {
    let mut hmac = HMAC::new(Algorithm::MD5, b"");
    let actual = hmac.finish().to_hex();
    assert_eq!(HMAC_MD5_EMPTY_STRING, actual)
}

#[test]
fn hmac_with_write() {
    let mut hmac = HMAC::new(Algorithm::MD5, b"key");
    hmac.write_all(TO_HASH.as_bytes()).expect("Could not write to hmac");
    let actual = hmac.finish().to_hex();
    assert_eq!(TO_HASH_HMAC_MD5, actual)
}

fn assert_hex_hashed_empty_string(algorithm: Algorithm, expected: &str) {
    assert_eq!(expected, hex_digest(algorithm, vec![]).as_str())
}

fn assert_hex_hmac_empty_string(algorithm: Algorithm, expected: &str) {
    assert_eq!(expected, hex_hmac(algorithm, vec![], vec![]).as_str())
}
