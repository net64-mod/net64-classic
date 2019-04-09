use super::types::*;

use libc::{c_char, c_float, c_int, c_void};

#[link(name = "mupen64plus")]
extern "C" {
    pub fn PluginStartup(
        core_lib_handle: *const c_void,
        context: *const c_void, // can be null
        debug_callback: *const extern "C" fn(*const c_void, c_int, *const c_char), // can be null
    ) -> M64pError;

    pub fn PluginShutdown() -> M64pError;

    pub fn PluginGetVersion(
        plugin_type: *const M64pPluginType,
        plugin_version: *const c_int,
        api_version: *const c_int,
        plugin_name: *const *const c_char,
        capabilities: *const c_int,
    ) -> M64pError;
}
