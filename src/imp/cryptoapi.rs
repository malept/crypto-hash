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

//! A cryptographic hash generator dependent upon Windows's `CryptoAPI`.
//!
//! Originally based on:
//! https://github.com/rust-lang/cargo/blob/0.10.0/src/cargo/util/sha256.rs
//! which is copyright (c) 2014 The Rust Project Developers under the MIT license.

use advapi32::{CryptAcquireContextW, CryptCreateHash, CryptDeriveKey, CryptDestroyHash,
               CryptDestroyKey, CryptGetHashParam, CryptHashData, CryptReleaseContext,
               CryptSetHashParam};
use std::io;
use std::mem;
use std::ptr;
use super::Algorithm;
use winapi::{ALG_ID, CALG_HMAC, CALG_MD5, CALG_RC4, CALG_SHA1, CALG_SHA_256, CALG_SHA_512,
             CRYPT_SILENT, CRYPT_VERIFYCONTEXT, DWORD, HCRYPTHASH, HCRYPTKEY, HCRYPTPROV, HMAC_INFO,
             HP_HASHVAL, HP_HMAC_INFO, PROV_RSA_AES};

macro_rules! call {
    ($e: expr) => ({
        if $e == 0 {
            panic!("failed {}: {}", stringify!($e), io::Error::last_os_error())
        }
    })
}

macro_rules! finish_algorithm {
    ($func_name: ident, $size: ident) => {
        fn $func_name(&mut self) -> Vec<u8> {
            let mut len = $size as u32;
            let mut hash = [0u8; $size];
            call!(unsafe {
                CryptGetHashParam(self.hcrypthash, HP_HASHVAL, hash.as_mut_ptr(), &mut len, 0)
            });
            assert_eq!(len as usize, hash.len());
            hash.to_vec()
        }
    }
}

const MD5_LENGTH: usize = 16;
const SHA1_LENGTH: usize = 20;
const SHA256_LENGTH: usize = 32;
const SHA512_LENGTH: usize = 64;

struct CryptHash {
    algorithm: Algorithm,
    hcryptprov: HCRYPTPROV,
    hcrypthash: HCRYPTHASH,
    hcryptkey: HCRYPTKEY,
}

impl CryptHash {
    fn new(algorithm: Algorithm, hmac_key: Option<&[u8]>) -> CryptHash {
        let hcp = CryptHash::acquire_context();
        let hash_type = match algorithm {
            Algorithm::MD5 => CALG_MD5,
            Algorithm::SHA1 => CALG_SHA1,
            Algorithm::SHA256 => CALG_SHA_256,
            Algorithm::SHA512 => CALG_SHA_512,
        };

        let algid = if hmac_key.is_some() {
            CALG_HMAC
        } else {
            hash_type
        };

        let hkey = match hmac_key {
            Some(key) => {
                CryptHash::generate_hmac_key(hcp, hash_type, key)
            }
            None => 0
        };

        let mut ret = CryptHash {
            algorithm: algorithm,
            hcryptprov: hcp,
            hcrypthash: 0,
            hcryptkey: hkey,
        };

        CryptHash::create(ret.hcryptprov, algid, hkey, &mut ret.hcrypthash);

        if hmac_key.is_some() {
            let hmac_info = HMAC_INFO {
                HashAlgid: hash_type,
                pbInnerString: 0 as *mut u8,
                cbInnerString: 0 as DWORD,
                pbOuterString: 0 as *mut u8,
                cbOuterString: 0 as DWORD,
            };

            call!(unsafe {
                CryptSetHashParam(ret.hcrypthash, HP_HMAC_INFO, mem::transmute(&hmac_info), 0)
            });
        }

        ret
    }

    fn acquire_context() -> HCRYPTPROV {
        let mut hcp = 0;
        call!(unsafe {
            CryptAcquireContextW(&mut hcp,
                                 ptr::null(),
                                 ptr::null(),
                                 PROV_RSA_AES,
                                 CRYPT_VERIFYCONTEXT | CRYPT_SILENT)
        });

        hcp
    }

    fn create(hcp: HCRYPTPROV, algid: ALG_ID, hkey: HCRYPTKEY, hcrypthash: &mut HCRYPTHASH) {
        call!(unsafe { CryptCreateHash(hcp, algid, hkey, 0, hcrypthash) });
    }

    fn update(hcrypthash: &mut HCRYPTHASH, buf: &[u8]) {
        call!(unsafe {
            CryptHashData(*hcrypthash,
                          buf.as_ptr() as *mut _,
                          buf.len() as DWORD,
                          0)
        });
    }

    fn generate_hmac_key(hcp: HCRYPTPROV, algid: ALG_ID, data: &[u8]) -> HCRYPTKEY {
        let mut hcrypthash: HCRYPTHASH = 0;
        CryptHash::create(hcp, algid, 0, &mut hcrypthash);
        CryptHash::update(&mut hcrypthash, data);
        let mut hkey = 0;
        let mut hkey_ptr = &mut hkey;
        assert_eq!(0, *hkey_ptr);
        call!(unsafe { CryptDeriveKey(hcp, CALG_RC4, hcrypthash, 0, hkey_ptr as *mut _) });
        assert!(*hkey_ptr != 0);

        *hkey_ptr
    }

    fn finish(&mut self) -> Vec<u8> {
        match self.algorithm {
            Algorithm::MD5 => self.finish_md5(),
            Algorithm::SHA1 => self.finish_sha1(),
            Algorithm::SHA256 => self.finish_sha256(),
            Algorithm::SHA512 => self.finish_sha512(),
        }
    }

    finish_algorithm!(finish_md5, MD5_LENGTH);
    finish_algorithm!(finish_sha1, SHA1_LENGTH);
    finish_algorithm!(finish_sha256, SHA256_LENGTH);
    finish_algorithm!(finish_sha512, SHA512_LENGTH);
}

impl io::Write for CryptHash {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        CryptHash::update(&mut self.hcrypthash, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for CryptHash {
    fn drop(&mut self) {
        if self.hcryptkey != 0 {
            call!(unsafe { CryptDestroyKey(self.hcryptkey) });
        }
        if self.hcrypthash != 0 {
            call!(unsafe { CryptDestroyHash(self.hcrypthash) });
        }
        call!(unsafe { CryptReleaseContext(self.hcryptprov, 0) });
    }
}

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
pub struct Hasher(CryptHash);

impl Hasher {
    /// Create a new `Hasher` for the given `Algorithm`.
    pub fn new(algorithm: Algorithm) -> Hasher {
        Hasher(CryptHash::new(algorithm, None))
    }

    /// Generate a digest from the data written to the `Hasher`.
    pub fn finish(&mut self) -> Vec<u8> {
        let Hasher(ref mut ch) = *self;
        ch.finish()
    }
}

impl io::Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let Hasher(ref mut ch) = *self;
        ch.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let Hasher(ref mut ch) = *self;
        ch.flush()
    }
}

pub struct HMAC(CryptHash);
impl HMAC {
    /// Create a new `HMAC` for the given `Algorithm`.
    pub fn new(algorithm: Algorithm, key: &[u8]) -> HMAC {
        HMAC(CryptHash::new(algorithm, Some(key)))
    }

    /// Generate a digest from the data written to the `HMAC`.
    pub fn finish(&mut self) -> Vec<u8> {
        let HMAC(ref mut ch) = *self;
        ch.finish()
    }
}

impl io::Write for HMAC {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let HMAC(ref mut ch) = *self;
        ch.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let HMAC(ref mut ch) = *self;
        ch.flush()
    }
}
