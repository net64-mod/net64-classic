use super::lib::{
    M64pCommand, M64pError, M64pPluginType, Mupen64Plus, Mupen64PlusPlugin, MupenError, Version,
};

use libc::{c_char, c_int, c_void};
use libloading::Library;

#[derive(Debug)]
pub struct MupenPlugin {
    lib: Mupen64PlusPlugin,
    version: Version,
}

impl MupenPlugin {
    pub fn new(mupen: &Mupen64Plus, lib: Mupen64PlusPlugin) -> Self {
        let version = Self::get_plugin_version(mupen, &lib)
            .expect("MupenPlugin::get_plugin_version() failed");
        dbg!(&version);
        MupenPlugin { lib, version }
    }

    pub fn startup(
        &self,
        core: &Mupen64Plus,
        context: *const c_void,
        debug_callback: Option<extern "C" fn(*const c_void, c_int, *const c_char)>,
    ) -> M64pError {
        self.lib.plugin_startup(core, context, debug_callback)
    }

    fn get_plugin_version(
        mupen: &Mupen64Plus,
        lib: &Mupen64PlusPlugin,
    ) -> Result<Version, MupenError> {
        let mut plugin_type = M64pPluginType::Core as M64pPluginType;
        let mut plugin_version = 0 as c_int;
        let mut api_version = 0 as c_int;
        let mut plugin_name = '0' as c_char;
        let mut capabilities = 0 as c_int;
        let m64p_error = lib.plugin_get_version(
            &mut plugin_type,
            &mut plugin_version,
            &mut api_version,
            &mut plugin_name,
            &mut capabilities,
        );
        match m64p_error {
            M64pError::Success => Ok(Version {
                plugin_type,
                plugin_version,
                api_version,
                plugin_name,
                capabilities,
            }),
            _ => Err(MupenError::new(mupen, m64p_error)),
        }
    }
}
