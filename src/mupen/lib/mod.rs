use libloading::Library;

mod config;
mod core;
mod error;
mod frontend;
mod plugin;
mod types;
mod video;

pub use self::config::*;
pub use self::core::*;
pub use self::error::*;
pub use self::frontend::*;
pub use self::plugin::*;
pub use self::types::*;
pub use self::video::*;

use libc::{c_char, c_int};

#[repr(C)]
pub struct Mupen64Plus {
    lib: Box<Library>,
}

impl From<Library> for Mupen64Plus {
    fn from(lib: Library) -> Self {
        Mupen64Plus { lib: Box::new(lib) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Mupen64PlusPlugin {
    lib: Library,
}

impl From<Library> for Mupen64PlusPlugin {
    fn from(lib: Library) -> Self {
        Mupen64PlusPlugin { lib }
    }
}

#[derive(Debug)]
pub struct Version {
    plugin_type: M64pPluginType,
    plugin_version: c_int,
    api_version: c_int,
    plugin_name: c_char,
    capabilities: c_int,
}

impl Version {
    pub fn new(
        plugin_type: M64pPluginType,
        plugin_version: c_int,
        api_version: c_int,
        plugin_name: c_char,
        capabilities: c_int,
    ) -> Self {
        Version {
            plugin_type,
            plugin_version,
            api_version,
            plugin_name,
            capabilities,
        }
    }
}
