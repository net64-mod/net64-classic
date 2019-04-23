use super::types::*;
use super::Mupen64Plus;

use libc::{c_char, c_int};
use libloading::Symbol;

type CoreGetAPIVersions = unsafe fn(*mut c_int, *mut c_int, *mut c_int, *mut c_int) -> M64pError;

type PluginGetVersion =
    unsafe fn(*mut M64pPluginType, *mut c_int, *mut c_int, *mut c_char, *mut c_int) -> M64pError;

type CoreErrorMessage = unsafe fn(m64p_error: M64pError) -> *const c_char;

impl Mupen64Plus {
    pub fn core_get_api_versions(
        &self,
        config_version: &mut c_int,
        debug_version: &mut c_int,
        vidext_version: &mut c_int,
        extra_version: &mut c_int,
    ) -> M64pError {
        unsafe {
            let core_get_api_version: Symbol<CoreGetAPIVersions> =
                self.lib.get(b"CoreGetAPIVersions").unwrap();

            core_get_api_version(config_version, debug_version, vidext_version, extra_version)
        }
    }

    pub fn plugin_get_version(
        &self,
        plugin_type: &mut M64pPluginType,
        plugin_version: &mut c_int,
        api_version: &mut c_int,
        plugin_name: &mut c_char,
        capabilities: &mut c_int,
    ) -> M64pError {
        unsafe {
            let plugin_get_version: Symbol<PluginGetVersion> =
                self.lib.get(b"PluginGetVersion").unwrap();

            plugin_get_version(
                plugin_type,
                plugin_version,
                api_version,
                plugin_name,
                capabilities,
            )
        }
    }

    pub fn core_error_message(&self, m64p_error: M64pError) -> *const c_char {
        unsafe {
            let core_error_message: Symbol<CoreErrorMessage> =
                self.lib.get(b"CoreErrorMessage").unwrap();

            core_error_message(m64p_error)
        }
    }
}
