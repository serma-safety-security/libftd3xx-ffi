//! Rust FFI bindings to the [FTDI D3XX drivers](https://www.ftdichip.com/drivers/d3xx-drivers).
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen")] {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        include!("bindings_linux_x86_64.rs");
    } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
        include!("bindings_windows_x86_64.rs");
    } else {
        std::compile_error!("Pre-generated bindings are not available for your target");
    }
}
