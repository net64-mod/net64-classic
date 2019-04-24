use super::types::*;
use super::{Mupen64Plus, Mupen64PlusPlugin};

use libc::{c_char, c_int, c_void};
use libloading::Symbol;

type PluginStartup = unsafe fn(
    *const c_void,
    *const c_void,
    Option<extern "C" fn(*const c_void, c_int, *const c_char)>,
) -> M64pError;

type PluginGetVersion =
    unsafe fn(*mut M64pPluginType, *mut c_int, *mut c_int, *mut c_char, *mut c_int) -> M64pError;

impl Mupen64PlusPlugin {
    pub fn plugin_startup(
        &self,
        core: &Mupen64Plus,
        context: *const c_void, // can be null
        debug_callback: Option<extern "C" fn(*const c_void, c_int, *const c_char)>,
    ) -> M64pError {
        unsafe {
            let plugin_startup: Symbol<PluginStartup> = self.lib.get(b"PluginStartup").unwrap();

            plugin_startup(core.lib.get_handle(), context, debug_callback)
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
}
