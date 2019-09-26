extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
fn link() {
    println!("cargo:rustc-link-search=cpp/lib/x86_64");

    println!("cargo:rustc-link-lib=static=filament");
    println!("cargo:rustc-link-lib=static=backend");
    println!("cargo:rustc-link-lib=static=bluegl");
    println!("cargo:rustc-link-lib=static=bluevk");
    println!("cargo:rustc-link-lib=static=filabridge");
    println!("cargo:rustc-link-lib=static=filaflat");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-lib=static=geometry");
    println!("cargo:rustc-link-lib=static=smol-v");
    println!("cargo:rustc-link-lib=static=ibl");

    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
}

#[cfg(target_os = "windows")]
fn link() {
    println!("cargo:rustc-link-search=cpp/lib/x86_64/mt");

    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=opengl32");

    println!("cargo:rustc-link-lib=static=filament");
    println!("cargo:rustc-link-lib=static=backend");
    println!("cargo:rustc-link-lib=static=bluegl");
    println!("cargo:rustc-link-lib=static=filabridge");
    println!("cargo:rustc-link-lib=static=filaflat");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-lib=static=geometry");
    println!("cargo:rustc-link-lib=static=smol-v");
    println!("cargo:rustc-link-lib=static=ibl");
}

#[cfg(target_os = "macos")]
fn cc_build() {
    cc::Build::new()
        .files(vec!["cpp/src/engine.cc"])
        .include("cpp/include")
        .cpp(true)
        .flag("-std=c++14")
        .static_flag(true)
        .compile("legion_filament_cpp");
}

#[cfg(target_os = "windows")]
fn cc_build() {
    cc::Build::new()
        .files(vec!["cpp/src/engine.cc"])
        .include("cpp/include")
        .cpp(true)
        .static_crt(true)
        .compile("legion_filament_cpp");
}

/// Use Bindgen to generate bindings from the `wrapper.h` header.
fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-Icpp/include")
        .header("cpp/src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    link();
    cc_build();
    generate_bindings();

    // Also re-run if any C++ source changes (useful for dev)
    println!("cargo:rerun-if-changed=cpp/src/**/*");
}