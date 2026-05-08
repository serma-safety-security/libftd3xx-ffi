//! Build script for the ftd3xx-ffi raw bindings.
//!
//! This build.rs script is based on the one made for the ftd2xx-ffi bindings
//! (https://github.com/ftdi-rs/libftd2xx-ffi/blob/main/build.rs).
//!
//! It links the appropriate libraries and also provide the ability to generate
//! bindings on the fly with bindgen using the `bingen` feature.
//!
//! The different supported OS and architecture should be extended as this crate
//! is currently WIP
//!
use std::{env, path::PathBuf};

// PathBuf is used so that the path segment separators match the host system
// This is important for cross-compiling for Windows from Linux


fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let mut header: PathBuf;
    let mut search: PathBuf = env::current_dir().unwrap();
    search.push("vendor");

    let link_lib: &str;

    match os.as_str() {
        "linux" => {
            match arch.as_str() {
                "x86_64" => (),
                tgt_arch => panic!("Architecture {} is not supported for linux", tgt_arch),
            }

            search.push(format!("linux-{}", arch));
            header = search.clone();

            header.push("ftd3xx.h");
            link_lib = if cfg!(feature = "static") {
                // FIXME: It seems rustc needs to dynamically link with stdc++ to avoid
                // "undefined reference to `vtable for __cxxabiv1::__class_type_info'" errors
                println!("cargo:rustc-link-search=native=/usr/lib");
                println!("cargo:rustc-link-lib=dylib=stdc++");

                "static=ftd3xx-static"
            } else {
                "dylib=ftd3xx"
            };
        },
        "windows" => {
            search.push("windows");

            header = search.clone();
            header.push("FTD3XX.h");

            match arch.as_str() {
                "x86" => (),
                "x86_64" => (),
                tgt_arch => panic!("Architecture {} is not supported for windows", tgt_arch),
            }

            search.push(format!("{}", arch));
    
            println!("cargo:rerun-if-env-changed=LIBMSVC_PATH");

            link_lib = if cfg!(feature = "static") {
                r"static=Static\FTD3XX"
            } else {
                "dylib=FTD3XX"
            };
        }
        other => panic!("OS {} is not supported", other),
    }

    println!(
        "cargo:rustc-link-search=native={}",
        search.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib={}", link_lib);

    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());

    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .allowlist_function("FT_.*")
            .allowlist_type("FT_.*")
            .allowlist_type("_FT_.*")
            .allowlist_var("FT_.*")
            .allowlist_type("CONFIGURATION_.*")
            //.rustfmt_bindings(true) // TODO: Check about this
            .derive_default(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings");
    }
}

