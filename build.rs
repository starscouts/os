use std::env;

fn main() {
    println!("cargo:rustc-env=DATE={:?}", chrono::offset::Utc::now());
    println!("cargo:rustc-env=CARGO_HOST={}", env::var("HOST").unwrap());
    println!("cargo:rustc-env=CARGO_TARGET={}", env::var("TARGET").unwrap());
    println!("cargo:rustc-env=CARGO_PROFILE={}", env::var("PROFILE").unwrap());
    println!("cargo:rustc-env=CARGO_RUSTC={}", env::var("RUSTC").unwrap());
}