use super::types::*;
use super::Mupen64Plus;

use libc::{c_char, c_int, c_void};
use libloading::Symbol;

type CoreStartup = unsafe fn(
    c_int,
    *const c_char,
    *const c_char,
    *const c_void,
    Option<extern "C" fn(*const c_void, c_int, *const c_char)>,
    *const c_void,
    *const extern "C" fn(*const c_void, c_int, c_int),
) -> M64pError;

type CoreShutdown = unsafe fn() -> M64pError;

type CoreAttachPlugin = unsafe fn(M64pPluginType, *const c_void) -> M64pError;

type CoreDetachPlugin = unsafe fn(M64pPluginType) -> M64pError;

type CoreDoCommand = unsafe fn(M64pCommand, c_int, *const c_void) -> M64pError;

impl Mupen64Plus {
    pub fn core_startup(
        &self,
        api_version: c_int,
        config_path: &c_char, // can be null
        data_path: &c_char,   // can be null
        context: &c_void,     // can be null
        debug_callback: Option<extern "C" fn(*const c_void, c_int, *const c_char)>, // can be null
        context2: &c_void,    // can be null
        state_callback: &extern "C" fn(*const c_void, c_int, c_int), // can be null
    ) -> M64pError {
        unsafe {
            let core_startup: Symbol<CoreStartup> = self.lib.get(b"CoreStartup").unwrap();

            core_startup(
                api_version,
                config_path,
                data_path,
                context,
                debug_callback,
                context2,
                state_callback,
            )
        }
    }

    pub fn core_shutdown(&self) -> M64pError {
        unsafe {
            let core_shutdown: Symbol<CoreShutdown> = self.lib.get(b"CoreShutdown").unwrap();

            core_shutdown()
        }
    }

    pub fn core_attach_plugin(
        &self,
        plugin_type: M64pPluginType,
        plugin_lib_handle: *const c_void,
    ) -> M64pError {
        unsafe {
            let core_attach_plugin: Symbol<CoreAttachPlugin> =
                self.lib.get(b"CoreAttachPlugin").unwrap();

            core_attach_plugin(plugin_type, plugin_lib_handle)
        }
    }

    pub fn core_detach_plugin(&self, plugin_type: M64pPluginType) -> M64pError {
        unsafe {
            let core_detach_plugin: Symbol<CoreDetachPlugin> =
                self.lib.get(b"CoreDetachPlugin").unwrap();

            core_detach_plugin(plugin_type)
        }
    }

    pub fn core_do_command(
        &self,
        command: M64pCommand,
        param_int: c_int,
        param_ptr: *const c_void,
    ) -> M64pError {
        unsafe {
            let core_do_command: Symbol<CoreDoCommand> = self.lib.get(b"CoreDoCommand").unwrap();

            core_do_command(command, param_int, param_ptr)
        }
    }
}
