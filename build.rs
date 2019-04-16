extern crate pkg_config;

fn main() {
    println!(r"cargo:rustc-link-search=/usr/lib");
    println!(r"cargo:rustc-link-lib=mupen64plus");
    // println!(r"cargo:rustc-link-search=/usr/lib/mupen64plus");
    // println!(r"cargo:rustc-link-lib=mupen64plus-audio-sdl");

    // pkg_config::probe_library("libmupen64plus.so").unwrap();
    // pkg_config::Config::new().atleast_version("1.1.4").probe("snappy").unwrap();
    // pkg_config::Config::new().atleast_version("2.5.0").probe("mupen64plus").unwrap();
}
