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

//! A cryptographic hash generator dependent upon OSX's `CommonCrypto`.

use libc::{c_int, c_uint, c_ulong, c_ulonglong};
use std::fmt;
use std::io;
use super::Algorithm;

macro_rules! unsafe_guard {
    ($e: expr) => {
        unsafe {
            let r = $e;
            assert_eq!(r, 1);
        }
    }
}

macro_rules! algorithm_helpers {
    (
        $ctx_ty: ident,
        $init_binding: ident,
        $write_binding: ident,
        $finish_binding: ident,
        $new_name: ident,
        $init_name: ident,
        $write_name: ident,
        $finish_name: ident,
        $hmac_finish_name: ident,
        $digest_len: ident
    ) => {
        fn $new_name() -> $ctx_ty {
            let mut ctx: $ctx_ty = $ctx_ty::new();
            $init_name(&mut ctx);
            ctx
        }

        fn $init_name(ctx: &mut $ctx_ty) {
            unsafe_guard!($init_binding(ctx));
            assert!(!(ctx as *mut $ctx_ty).is_null());
        }

        fn $write_name(ctx: &mut $ctx_ty, buf: &[u8]) {
            unsafe_guard!($write_binding(ctx, buf.as_ptr(), buf.len()));
        }

        fn $finish_name(ctx: &mut $ctx_ty) -> Vec<u8> {
            let mut md = [0u8; $digest_len];
            unsafe_guard!($finish_binding(md.as_mut_ptr(), ctx));
            md.to_vec()
        }

        fn $hmac_finish_name(ctx: &mut CCHmacContext) -> Vec<u8> {
            let mut hmac = [0u8; $digest_len];
            unsafe { CCHmacFinal(ctx, hmac[..].as_mut_ptr()); }
            hmac.to_vec()
        }
    }
}

const MD5_CBLOCK: usize = 64;
const MD5_LBLOCK: usize = MD5_CBLOCK / 4;
const MD5_DIGEST_LENGTH: usize = 16;

const SHA_LBLOCK: usize = 16;
const SHA1_DIGEST_LENGTH: usize = 20;
const SHA256_DIGEST_LENGTH: usize = 32;
const SHA512_DIGEST_LENGTH: usize = 64;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
struct CC_MD5_CTX {
    A: c_uint,
    B: c_uint,
    C: c_uint,
    D: c_uint,
    Nl: c_uint,
    Nh: c_uint,
    data: [c_uint; MD5_LBLOCK],
    num: c_uint,
}

impl CC_MD5_CTX {
    fn new() -> CC_MD5_CTX {
        CC_MD5_CTX {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            Nl: 0,
            Nh: 0,
            data: [0 as c_uint; MD5_LBLOCK],
            num: 0,
        }
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
struct CC_SHA_CTX {
    h0: c_uint,
    h1: c_uint,
    h2: c_uint,
    h3: c_uint,
    h4: c_uint,
    Nl: c_uint,
    Nh: c_uint,
    data: [c_uint; SHA_LBLOCK],
    num: c_uint,
}

impl CC_SHA_CTX {
    fn new() -> CC_SHA_CTX {
        CC_SHA_CTX {
            h0: 0,
            h1: 0,
            h2: 0,
            h3: 0,
            h4: 0,
            Nl: 0,
            Nh: 0,
            data: [0 as c_uint; SHA_LBLOCK],
            num: 0,
        }
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
struct CC_SHA256_CTX {
    h: [c_ulong; 8],
    Nl: c_ulong,
    Nh: c_ulong,
    data: [c_ulong; SHA_LBLOCK],
    num: c_uint,
    md_len: c_uint,
}

impl CC_SHA256_CTX {
    fn new() -> CC_SHA256_CTX {
        CC_SHA256_CTX {
            h: [0 as c_ulong; 8],
            Nl: 0,
            Nh: 0,
            data: [0 as c_ulong; SHA_LBLOCK],
            num: 0,
            md_len: 0,
        }
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
struct CC_SHA512_CTX {
    h: [c_ulonglong; 8],
    Nl: c_ulonglong,
    Nh: c_ulonglong,
    data: [c_ulonglong; SHA_LBLOCK],
    num: c_uint,
    md_len: c_uint,
}

impl CC_SHA512_CTX {
    fn new() -> CC_SHA512_CTX {
        CC_SHA512_CTX {
            h: [0 as c_ulonglong; 8],
            Nl: 0,
            Nh: 0,
            data: [0 as c_ulonglong; SHA_LBLOCK],
            num: 0,
            md_len: 0,
        }
    }
}

#[derive(Debug)]
enum DigestContext {
    MD5(CC_MD5_CTX),
    SHA1(CC_SHA_CTX),
    SHA256(CC_SHA256_CTX),
    SHA512(CC_SHA512_CTX),
}

#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
enum CCHmacAlgorithm {
    kCCHmacAlgSHA1,
    kCCHmacAlgMD5,
    kCCHmacAlgSHA256,
    kCCHmacAlgSHA384,
    kCCHmacAlgSHA512,
    kCCHmacAlgSHA224,
}

const CC_HMAC_CONTEXT_SIZE: usize = 96;

#[allow(non_camel_case_types, non_snake_case)]
#[repr(C)]
struct CCHmacContext {
    ctx: [u32; CC_HMAC_CONTEXT_SIZE],
}

impl fmt::Debug for CCHmacContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("CCHmacContext {{ ctx: [");
        let mut first = true;
        for i in 0..CC_HMAC_CONTEXT_SIZE {
            let item = self.ctx[i];
            write!(output, "{}", item);
            if first {
                write!(output, ", ");
                first = false;
            }
        }
        output.push_str("] }}");
        write!(f, "{}", output)
    }
}

impl CCHmacContext {
    fn new() -> CCHmacContext {
        CCHmacContext {
            ctx: [0u32; CC_HMAC_CONTEXT_SIZE],
        }
    }
}

extern "C" {
    fn CC_MD5_Init(ctx: *mut CC_MD5_CTX) -> c_int;
    fn CC_MD5_Update(ctx: *mut CC_MD5_CTX, data: *const u8, n: usize) -> c_int;
    fn CC_MD5_Final(md: *mut u8, ctx: *mut CC_MD5_CTX) -> c_int;
    fn CC_SHA1_Init(ctx: *mut CC_SHA_CTX) -> c_int;
    fn CC_SHA1_Update(ctx: *mut CC_SHA_CTX, data: *const u8, n: usize) -> c_int;
    fn CC_SHA1_Final(md: *mut u8, ctx: *mut CC_SHA_CTX) -> c_int;
    fn CC_SHA256_Init(ctx: *mut CC_SHA256_CTX) -> c_int;
    fn CC_SHA256_Update(ctx: *mut CC_SHA256_CTX, data: *const u8, n: usize) -> c_int;
    fn CC_SHA256_Final(md: *mut u8, ctx: *mut CC_SHA256_CTX) -> c_int;
    fn CC_SHA512_Init(ctx: *mut CC_SHA512_CTX) -> c_int;
    fn CC_SHA512_Update(ctx: *mut CC_SHA512_CTX, data: *const u8, n: usize) -> c_int;
    fn CC_SHA512_Final(md: *mut u8, ctx: *mut CC_SHA512_CTX) -> c_int;
    fn CCHmacInit(ctx: *mut CCHmacContext,
                  algorithm: CCHmacAlgorithm,
                  key: *const u8,
                  keyLength: usize);
    fn CCHmacUpdate(ctx: *mut CCHmacContext, data: *const u8, dataLength: usize);
    fn CCHmacFinal(ctx: *mut CCHmacContext, macOut: *mut u8);
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    Reset,
    Updated,
    Finalized,
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
#[derive(Debug)]
pub struct Hasher {
    context: DigestContext,
    state: State,
}

#[derive(Debug)]
pub struct HMAC {
    algorithm: Algorithm,
    context: CCHmacContext,
}

algorithm_helpers!(CC_MD5_CTX,
                   CC_MD5_Init,
                   CC_MD5_Update,
                   CC_MD5_Final,
                   md5_new,
                   md5_init,
                   md5_write,
                   md5_finish,
                   hmac_md5_finish,
                   MD5_DIGEST_LENGTH);
algorithm_helpers!(CC_SHA_CTX,
                   CC_SHA1_Init,
                   CC_SHA1_Update,
                   CC_SHA1_Final,
                   sha1_new,
                   sha1_init,
                   sha1_write,
                   sha1_finish,
                   hmac_sha1_finish,
                   SHA1_DIGEST_LENGTH);
algorithm_helpers!(CC_SHA256_CTX,
                   CC_SHA256_Init,
                   CC_SHA256_Update,
                   CC_SHA256_Final,
                   sha256_new,
                   sha256_init,
                   sha256_write,
                   sha256_finish,
                   hmac_sha256_finish,
                   SHA256_DIGEST_LENGTH);
algorithm_helpers!(CC_SHA512_CTX,
                   CC_SHA512_Init,
                   CC_SHA512_Update,
                   CC_SHA512_Final,
                   sha512_new,
                   sha512_init,
                   sha512_write,
                   sha512_finish,
                   hmac_sha512_finish,
                   SHA512_DIGEST_LENGTH);

impl Hasher {
    /// Create a new `Hasher` for the given `Algorithm`.
    pub fn new(algorithm: Algorithm) -> Hasher {
        let context = match algorithm {
            Algorithm::MD5 => DigestContext::MD5(md5_new()),
            Algorithm::SHA1 => DigestContext::SHA1(sha1_new()),
            Algorithm::SHA256 => DigestContext::SHA256(sha256_new()),
            Algorithm::SHA512 => DigestContext::SHA512(sha512_new()),
        };

        Hasher {
            context: context,
            state: State::Reset,
        }
    }

    fn init(&mut self) {
        match self.state {
            State::Reset => return,
            State::Updated => {
                self.finish();
            }
            State::Finalized => (),
        }
        match self.context {
            DigestContext::MD5(ref mut ctx) => md5_init(ctx),
            DigestContext::SHA1(ref mut ctx) => sha1_init(ctx),
            DigestContext::SHA256(ref mut ctx) => sha256_init(ctx),
            DigestContext::SHA512(ref mut ctx) => sha512_init(ctx),
        }
        self.state = State::Reset;
    }

    /// Generate a digest from the data written to the `Hasher`.
    pub fn finish(&mut self) -> Vec<u8> {
        if self.state == State::Finalized {
            self.init();
        }
        let result = match self.context {
            DigestContext::MD5(ref mut ctx) => md5_finish(ctx),
            DigestContext::SHA1(ref mut ctx) => sha1_finish(ctx),
            DigestContext::SHA256(ref mut ctx) => sha256_finish(ctx),
            DigestContext::SHA512(ref mut ctx) => sha512_finish(ctx),
        };
        self.state = State::Finalized;

        result
    }
}

impl io::Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.state == State::Finalized {
            self.init();
        }
        match self.context {
            DigestContext::MD5(ref mut ctx) => md5_write(ctx, buf),
            DigestContext::SHA1(ref mut ctx) => sha1_write(ctx, buf),
            DigestContext::SHA256(ref mut ctx) => sha256_write(ctx, buf),
            DigestContext::SHA512(ref mut ctx) => sha512_write(ctx, buf),
        }
        self.state = State::Updated;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for Hasher {
    fn drop(&mut self) {
        if self.state != State::Finalized {
            self.finish();
        }
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
        let mut ctx = CCHmacContext::new();
        let hmac_algorithm = algorithm_to_hmac_type(&algorithm);
        unsafe { CCHmacInit(&mut ctx, hmac_algorithm, key.as_ptr(), key.len()); }
        HMAC {
            algorithm: algorithm,
            context: ctx,
        }
    }

    /// Generate an HMAC from the key + data written to the `HMAC` instance.
    pub fn finish(&mut self) -> Vec<u8> {
        match self.algorithm {
            Algorithm::MD5 => hmac_md5_finish(&mut self.context),
            Algorithm::SHA1 => hmac_sha1_finish(&mut self.context),
            Algorithm::SHA256 => hmac_sha256_finish(&mut self.context),
            Algorithm::SHA512 => hmac_sha512_finish(&mut self.context),
        }
    }
}

impl io::Write for HMAC {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { CCHmacUpdate(&mut self.context, buf.as_ptr(), buf.len()); }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
