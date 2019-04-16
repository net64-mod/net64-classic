use crate::platform::{DEFAULT_DYNLIB, LIB_SEARCH_DIRS, PLUGIN_SEARCH_DIRS};

use super::lib::{M64pCommand, M64pError, Mupen64Plus, Mupen64PlusPlugin, MupenError};

use libc::{c_char, c_int, c_void};
use libloading::Library;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

static SM64_ROM: &[u8] = include_bytes!("../../Super Mario 64 (USA).n64");

pub struct Core {
    lib: Mupen64Plus,
    plugins: Plugins,
    versions: Versions,
}

#[derive(Debug)]
struct Plugins {
    audio: Mupen64PlusPlugin,
    input: Mupen64PlusPlugin,
    rsp: Mupen64PlusPlugin,
    video: Mupen64PlusPlugin,
}

#[derive(Debug)]
struct Versions {
    config_version: c_int,
    debug_version: c_int,
    vidext_version: c_int,
    extra_version: c_int,
}

impl Core {
    pub fn new() -> Self {
        let lib = Self::load_lib().expect("Core::load_lib() failed");
        let plugins = Self::load_plugins().expect("Core::load_plugins() failed");
        let versions = Self::get_api_versions(&lib).expect("Core::get_api_versions() failed");
        Self {
            lib,
            plugins,
            versions,
        }
    }

    fn load_lib() -> Result<Mupen64Plus, io::Error> {
        for search_dir in LIB_SEARCH_DIRS.iter() {
            let mut path: PathBuf = search_dir.into();
            path.push(DEFAULT_DYNLIB);
            if path.exists() {
                return Ok(Library::new(path).unwrap().into());
            }
        }
        panic!("Mupen64Plus library not found");
    }

    fn load_plugins() -> Result<Plugins, io::Error> {
        let mut audio_plugin = Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Audio plugin not found",
        ));
        let mut input_plugin = Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input plugin not found",
        ));
        let mut rsp_plugin = Err(io::Error::new(
            io::ErrorKind::NotFound,
            "RSP plugin not found",
        ));
        let mut video_plugin = Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Video plugin not found",
        ));

        for search_dir in PLUGIN_SEARCH_DIRS.iter() {
            match read_dir(search_dir) {
                Ok(dir) => {
                    for entry in dir {
                        let entry = entry?;
                        let path = entry.path();
                        if !path.is_dir() {
                            if audio_plugin.is_err() {
                                if path.to_str().unwrap().ends_with("mupen64plus-audio-sdl.so") {
                                    audio_plugin = Ok(path);
                                    continue;
                                }
                            }
                            if input_plugin.is_err() {
                                if path.to_str().unwrap().ends_with("mupen64plus-input-sdl.so") {
                                    input_plugin = Ok(path);
                                    continue;
                                }
                            }
                            if rsp_plugin.is_err() {
                                if path.to_str().unwrap().ends_with("mupen64plus-rsp-hle.so") {
                                    rsp_plugin = Ok(path);
                                    continue;
                                }
                            }
                            if video_plugin.is_err() {
                                if path
                                    .to_str()
                                    .unwrap()
                                    .ends_with("mupen64plus-video-glide64mk2.so")
                                {
                                    video_plugin = Ok(path);
                                    continue;
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
        return Ok(Plugins {
            audio: Library::new(audio_plugin?).unwrap().into(),
            input: Library::new(input_plugin?).unwrap().into(),
            rsp: Library::new(rsp_plugin?).unwrap().into(),
            video: Library::new(video_plugin?).unwrap().into(),
        });
    }

    fn get_api_versions(lib: &Mupen64Plus) -> Result<Versions, MupenError> {
        let mut config_version = 0 as c_int;
        let mut debug_version = 0 as c_int;
        let mut vidext_version = 0 as c_int;
        let mut extra_version = 0 as c_int;
        let m64p_error = lib.core_get_api_versions(
            &mut config_version,
            &mut debug_version,
            &mut vidext_version,
            &mut extra_version,
        );

        match m64p_error {
            M64pError::Success => Ok(Versions {
                config_version,
                debug_version,
                vidext_version,
                extra_version,
            }),
            _ => Err(MupenError::new(&lib, m64p_error)),
        }
    }

    pub fn startup(&self) -> Result<(), MupenError> {
        dbg!(&self.versions);
        println!("{:x}", &self.versions.config_version);
        let config_path = std::ptr::null() as *const c_char;
        let data_path = std::ptr::null() as *const c_char;
        let context = std::ptr::null() as *const c_void;
        let debug_callback = None;
        let context2 = std::ptr::null() as *const c_void;
        let state_callback = std::ptr::null();
        unsafe {
            let m64p_error = self.lib.core_startup(
                131841, // self.versions.config_version,
                &*config_path,
                &*data_path,
                &*context,
                debug_callback,
                &*context2,
                &*state_callback,
            );
            match m64p_error {
                M64pError::Success => Ok(()),
                _ => Err(MupenError::new(&self.lib, m64p_error)),
            }
        }
    }

    pub fn startup_plugins(&self) -> Result<(), MupenError> {
        self.startup_video_plugin()?;
        // self.startup_audio_plugin()?;
        // self.startup_input_plugin()?;
        // self.startup_rsp_plugin()?;
        Ok(())
    }

    fn startup_video_plugin(&self) -> Result<(), MupenError> {
        let context = std::ptr::null();
        let debug_callback = None;
        let m64p_error = self
            .plugins
            .video
            .plugin_startup(&self.lib, context, debug_callback);
        match m64p_error {
            M64pError::Success => Ok(()),
            _ => Err(MupenError::new(&self.lib, m64p_error)),
        }
    }

    // fn startup_audio_plugin(&self) -> Result<(), MupenError> {
    //     let context = std::ptr::null();
    //     let debug_callback = None;
    //     let m64p_error = self
    //         .plugins
    //         .audio
    //         .plugin_startup(&self.lib, context, debug_callback);
    //     match m64p_error {
    //         M64pError::Success => Ok(()),
    //         _ => Err(MupenError::new(&self.lib, m64p_error)),
    //     }
    // }

    // fn startup_input_plugin(&self) -> Result<(), MupenError> {
    //     let context = std::ptr::null();
    //     let debug_callback = None;
    //     let m64p_error = self
    //         .plugins
    //         .input
    //         .plugin_startup(&self.lib, context, debug_callback);
    //     match m64p_erroLibraryr {
    //         M64pError::Success => Ok(()),
    //         _ => Err(MupenError::new(&self.lib, m64p_error)),
    //     }
    // }

    // fn startup_rsp_plugin(&self) -> Result<(), MupenError> {
    //     let context = std::ptr::null();
    //     let debug_callback = None;
    //     let m64p_error = self
    //         .plugins
    //         .rsp
    //         .plugin_startup(&self.lib, context, debug_callback);
    //     match m64p_error {
    //         M64pError::Success => Ok(()),
    //         _ => Err(MupenError::new(&self.lib, m64p_error)),
    //     }
    // }

    pub fn open_rom(&self) -> Result<(), MupenError> {
        let m64p_error = self.lib.core_do_command(
            M64pCommand::RomOpen,
            SM64_ROM.len() as c_int,
            SM64_ROM.as_ptr() as *const c_void,
        );
        match m64p_error {
            M64pError::Success => Ok(()),
            _ => Err(MupenError::new(&self.lib, m64p_error)),
        }
    }

    // pub fn plugin_startup(&self) -> Result<(), MupenError> {
    //     unsafe {
    //         let m64p_error = PluginStartup(
    //             self.versions.config_version,
    //             config_path,
    //             data_path,
    //             context,
    //             debug_callback,
    //             context2,
    //             state_callback,
    //         );
    //         match m64p_error {
    //             M64pError::Success => Ok(()),
    //             _ => Err(m64p_error.into()),
    //         }
    //     }
    // }
}
