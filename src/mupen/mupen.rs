use super::error::MupenError;
use super::lib::{CoreDoCommand, CoreGetAPIVersions, CoreStartup, M64pCommand, M64pError};

use libc::{c_char, c_int, c_void};

static SM64_ROM: &[u8] = include_bytes!("../../Super Mario 64 (USA).n64");

pub struct Core {
    versions: Versions,
}

struct Versions {
    config_version: c_int,
    debug_version: c_int,
    vidext_version: c_int,
    extra_version: c_int,
}

impl Core {
    pub fn new() -> Self {
        Self {
            versions: Self::get_api_versions().unwrap(),
        }
    }

    fn get_api_versions() -> Result<Versions, MupenError> {
        let config_version = 0 as c_int;
        let debug_version = 0 as c_int;
        let vidext_version = 0 as c_int;
        let extra_version = 0 as c_int;
        unsafe {
            let m64p_error = CoreGetAPIVersions(
                config_version as *mut c_int,
                debug_version as *mut c_int,
                vidext_version as *mut c_int,
                extra_version as *mut c_int,
            );
            match m64p_error {
                M64pError::Success => Ok(Versions {
                    config_version,
                    debug_version,
                    vidext_version,
                    extra_version,
                }),
                _ => Err(m64p_error.into()),
            }
        }
    }

    pub fn startup(&self) -> Result<(), MupenError> {
        let config_path = std::ptr::null() as *const c_char;
        let data_path = std::ptr::null() as *const c_char;
        let context = std::ptr::null() as *const c_void;
        let debug_callback = std::ptr::null();
        let context2 = std::ptr::null() as *const c_void;
        let state_callback = std::ptr::null();
        unsafe {
            let m64p_error = CoreStartup(
                self.versions.config_version,
                config_path,
                data_path,
                context,
                debug_callback,
                context2,
                state_callback,
            );
            match m64p_error {
                M64pError::Success => Ok(()),
                _ => Err(m64p_error.into()),
            }
        }
    }

    pub fn open_rom() -> Result<(), MupenError> {
        unsafe {
            let m64p_error = CoreDoCommand(
                M64pCommand::RomOpen,
                SM64_ROM.len() as c_int,
                SM64_ROM.as_ptr() as *const c_void,
            );
            match m64p_error {
                M64pError::Success => Ok(()),
                _ => Err(m64p_error.into()),
            }
        }
    }
}
