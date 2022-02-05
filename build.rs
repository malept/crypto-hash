use std::env;

fn main() {
    if env::var("CRYPTO_HASH_FORCE_OPENSSL").is_ok() {
        println!("cargo:rustc-cfg=feature=\"openssl\"");
        return;
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    println!("cargo:rustc-cfg=feature=\"commoncrypto\"");

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-cfg=feature=\"winapi\"");

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios")))']
    println!("cargo:rustc-cfg=feature=\"openssl\"");
}
