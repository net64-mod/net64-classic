use super::types::*;

use libc::{c_char, c_int, c_void};

#[link(name = "mupen64plus")]
extern "C" {
    pub fn CoreStartup(
        api_version: c_int,
        config_path: *const c_char, // can be null
        data_path: *const c_char,   // can be null
        context: *const c_void,     // can be null
        debug_callback: *const extern "C" fn(*const c_void, c_int, *const c_char), // can be null
        context2: *const c_void,    // can be null
        state_callback: *const extern "C" fn(*const c_void, c_int, c_int), // can be null
    ) -> M64pError;

    pub fn CoreShutdown() -> M64pError;

    pub fn CoreAttachPlugin(
        plugin_type: M64pPluginType,
        plugin_lib_handle: *const c_void,
    ) -> M64pError;

    pub fn CoreDetachPlugin(plugin_type: M64pPluginType) -> M64pError;

    pub fn CoreDoCommand(
        command: M64pCommand,
        param_int: c_int,
        param_ptr: *const c_void,
    ) -> M64pError;

    pub fn VidExt_Init() -> M64pError;

    pub fn VidExt_Quit() -> M64pError;
}
