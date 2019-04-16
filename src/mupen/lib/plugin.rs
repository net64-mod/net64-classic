use super::types::*;
use super::{Mupen64Plus, Mupen64PlusPlugin};

use libc::{c_char, c_int, c_void};
use libloading::{Library, Symbol};

type PluginStartup = unsafe fn(
    *const Library,
    *const c_void,
    Option<extern "C" fn(*const c_void, c_int, *const c_char)>,
) -> M64pError;

impl Mupen64PlusPlugin {
    pub fn plugin_startup(
        &self,
        core: &Mupen64Plus,
        context: *const c_void, // can be null
        debug_callback: Option<extern "C" fn(*const c_void, c_int, *const c_char)>,
    ) -> M64pError {
        unsafe {
            let plugin_startup: Symbol<PluginStartup> = self.lib.get(b"PluginStartup").unwrap();

            plugin_startup(&core.lib, context, debug_callback)
        }
    }
}
