#[cfg(target_os = "linux")]
static DEFAULT_DYNLIB: &str = "libmupen64plus.so.2";

#[cfg(target_os = "linux")]
static DLL_EXT: &str = ".so";

#[cfg(target_os = "linux")]
static DLL_FILTER: &str = ".so.2";

#[cfg(target_os = "linux")]
static SEARCH_DIRS: [&str; 8] = [
    "/usr/local/lib/mupen64plus",
    "/usr/lib64/mupen64plus",
    "/usr/lib/mupen64plus",
    "/usr/games/lib64/mupen64plus",
    "/usr/games/lib/mupen64plus",
    "/usr/lib/x86_64-linux-gnu/mupen64plus",
    "/usr/lib/i386-linux-gnu/mupen64plus",
    ".",
];

#[cfg(target_os = "macos")]
static DEFAULT_DYNLIB: &str = "libmupen64plus.dylib";

#[cfg(target_os = "macos")]
static DLL_EXT: &str = ".dylib";

#[cfg(target_os = "macos")]
static DLL_FILTER: &str = ".dylib";

#[cfg(target_os = "macos")]
static SEARCH_DIRS: [&str; 3] = ["/usr/local/lib/mupen64plus", "/usr/lib/mupen64plus", "."];

#[cfg(target_os = "windows")]
static DEFAULT_DYNLIB: &str = "mupen64plus.dll";

#[cfg(target_os = "windows")]
static DLL_EXT: &str = ".dll";

#[cfg(target_os = "windows")]
static DLL_FILTER: &str = ".dll";

#[cfg(target_os = "windows")]
static SEARCH_DIRS: [&str; 1] = ["."];
