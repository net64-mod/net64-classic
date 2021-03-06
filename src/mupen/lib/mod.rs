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

use libc::{c_char, c_int, c_void};

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

impl Mupen64PlusPlugin {
    pub fn get_handle(&self) -> *const c_void {
        self.lib.get_handle()
    }
}

impl From<Library> for Mupen64PlusPlugin {
    fn from(lib: Library) -> Self {
        Mupen64PlusPlugin { lib }
    }
}

#[derive(Debug)]
pub struct Version {
    pub plugin_type: M64pPluginType,
    pub plugin_version: c_int,
    pub api_version: c_int,
    pub plugin_name: c_char,
    pub capabilities: c_int,
}
