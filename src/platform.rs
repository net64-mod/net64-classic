#[cfg(target_os = "linux")]
pub static DEFAULT_DYNLIB: &str = "libmupen64plus.so.2";

#[cfg(target_os = "linux")]
pub static DLL_EXT: &str = ".so";

#[cfg(target_os = "linux")]
pub static DLL_FILTER: &str = ".so.2";

#[cfg(target_os = "linux")]
pub static LIB_SEARCH_DIRS: [&str; 1] = [
    // "./lib/mupen64plus-bundle-linux64-2.5",
    "/usr/lib",
    // "/usr/lib64",
    // "/lib",
    // "/lib64",
    // ".",
];

#[cfg(target_os = "linux")]
pub static PLUGIN_SEARCH_DIRS: [&str; 1] = [
    // "./lib/mupen64plus-bundle-linux64-2.5"
    // "/usr/local/lib/mupen64plus",
    // "/usr/lib64/mupen64plus",
    "/usr/lib/mupen64plus",
    // "/usr/games/lib64/mupen64plus",
    // "/usr/games/lib/mupen64plus",
    // "/usr/lib/x86_64-linux-gnu/mupen64plus",
    // "/usr/lib/i386-linux-gnu/mupen64plus",
    // ".",
];

#[cfg(target_os = "macos")]
pub static DEFAULT_DYNLIB: &str = "libmupen64plus.dylib";

#[cfg(target_os = "macos")]
pub static DLL_EXT: &str = ".dylib";

#[cfg(target_os = "macos")]
pub static DLL_FILTER: &str = ".dylib";

#[cfg(target_os = "macos")]
pub static PLUGIN_SEARCH_DIRS: [&str; 3] =
    ["/usr/local/lib/mupen64plus", "/usr/lib/mupen64plus", "."];

#[cfg(target_os = "windows")]
pub static DEFAULT_DYNLIB: &str = "mupen64plus.dll";

#[cfg(target_os = "windows")]
pub static DLL_EXT: &str = ".dll";

#[cfg(target_os = "windows")]
pub static DLL_FILTER: &str = ".dll";

#[cfg(target_os = "windows")]
pub static PLUGIN_SEARCH_DIRS: [&str; 1] = ["."];
