extern crate bindgen;
extern crate cc;
extern crate flate2;
extern crate tar;

use flate2::read::GzDecoder;
use std::{
    env,
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

// Prebuilt tarball locations
#[cfg(target_os = "windows")]
const LATEST_PREBUILT_URL: &'static str =
    "https://github.com/google/filament/releases/download/v1.3.2/filament-20190827-windows.tgz";

#[cfg(target_os = "macos")]
const LATEST_PREBUILT_URL: &'static str =
    "https://github.com/google/filament/releases/download/v1.3.2/filament-20190826-mac.tgz";

// Returns the location of the downloaded tarball
fn download_prebuilt() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();

    let archive_name = LATEST_PREBUILT_URL.split("/").last().unwrap();
    let build_path = Path::new(&out_dir).join("filament_prebuilt");
    let archive_path = Path::new(&out_dir).join(archive_name);
    let extracted_done_marker = build_path.join("extract_done_marker");

    create_dir_all(&build_path).unwrap();

    // Avoid re-downloading the archive if needed.
    if !archive_path.exists() {
        download_to(LATEST_PREBUILT_URL, archive_path.to_str().unwrap());
    }

    // Only extract if the marker is missing
    if !extracted_done_marker.exists() {
        let reader = GzDecoder::new(File::open(&archive_path).unwrap());
        let mut ar = tar::Archive::new(reader);
        ar.unpack(&build_path).unwrap();

        // Create a marker that the archive was extracted
        File::create(extracted_done_marker).unwrap();
    }

    if cfg!(target_os = "macos") {
        // The OSX path isn't the same as windows. Which is neat.
        build_path.join("filament")
    } else {
        build_path
    }
}

fn download_to(url: &str, dest: &str) {
    if cfg!(windows) {
        run_command(
            "powershell",
            &[
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &format!(
                    "& {{
                $client = New-Object System.Net.WebClient
                $client.DownloadFile(\"{0}\", \"{1}\")
                if (!$?) {{ Exit 1 }}
            }}",
                    url, dest
                )
                .as_str(),
            ],
        );
    } else {
        // The -L will follow redirects.
        run_command("curl", &["-L", url, "-o", dest]);
    }
}

fn run_command(cmd: &str, args: &[&str]) {
    use std::process::Command;
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if !output.status.success() {
                let error = std::str::from_utf8(&output.stderr).unwrap();
                panic!("Command '{}' failed: {}", cmd, error);
            }
        }
        Err(error) => {
            panic!("Error running command '{}': {:#}", cmd, error);
        }
    }
}

#[cfg(target_os = "macos")]
fn link(build_path: PathBuf) {
    println!(
        "cargo:rustc-link-search={}",
        build_path.join("lib/x86_64").to_str().unwrap()
    );

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
fn link(build_path: PathBuf) {
    println!(
        "cargo:rustc-link-search={}",
        build_path.join("lib/x86_64/mt").to_str().unwrap()
    );

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
fn cc_build(build_path: PathBuf, source: Vec<&str>) {
    cc::Build::new()
        .files(source)
        .include(build_path.join("include").to_str().unwrap())
        .cpp(true)
        .flag("-std=c++14")
        .static_flag(true)
        .warnings(false)
        .compile("legion_filament_cpp");
}

#[cfg(target_os = "windows")]
fn cc_build(build_path: PathBuf, source: Vec<&str>) {
    cc::Build::new()
        .files(source)
        .include(build_path.join("include").to_str().unwrap())
        .cpp(true)
        .static_crt(true)
        .warnings(false)
        .compile("legion_filament_cpp");
}

/// Use Bindgen to generate bindings from the `wrapper.h` header.
fn generate_bindings(build_path: PathBuf) {
    let bindings = bindgen::Builder::default()
        .clang_arg(format!(
            "-I{}",
            build_path.join("include").to_str().unwrap()
        ))
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
    let prebuilt_path = download_prebuilt();

    let source = vec![
        "cpp/src/camera.cc",
        "cpp/src/engine.cc",
        "cpp/src/entity_manager.cc",
        "cpp/src/index_buffer.cc",
        "cpp/src/material.cc",
        "cpp/src/renderable_manager.cc",
        "cpp/src/renderer.cc",
        "cpp/src/scene.cc",
        "cpp/src/texture.cc",
        "cpp/src/transform_manager.cc",
        "cpp/src/vertex_buffer.cc",
        "cpp/src/view.cc",
    ];

    link(prebuilt_path.clone());
    cc_build(prebuilt_path.clone(), source);
    generate_bindings(prebuilt_path.clone());
}
