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

pub struct Mupen64Plus {
    lib: Library,
}

impl From<Library> for Mupen64Plus {
    fn from(lib: Library) -> Self {
        Mupen64Plus { lib }
    }
}

#[derive(Debug)]
pub struct Mupen64PlusPlugin {
    lib: Library,
}

impl From<Library> for Mupen64PlusPlugin {
    fn from(lib: Library) -> Self {
        Mupen64PlusPlugin { lib }
    }
}
