use super::types::*;

use libc::{c_char, c_int};

#[link(name = "mupen64plus")]
extern "C" {
    pub fn PluginGetVersion(
        plugin_type: *const M64pPluginType,
        plugin_version: *const c_int,
        api_version: *const c_int,
        plugin_name: *const *const c_char,
        capabilities: *const c_int,
    ) -> M64pError;

    pub fn CoreGetAPIVersions(
        config_version: *mut c_int,
        debug_version: *mut c_int,
        vidext_version: *mut c_int,
        extra_version: *mut c_int,
    ) -> M64pError;

    pub fn CoreErrorMessage(m64p_error: M64pError) -> *const c_char;
}
