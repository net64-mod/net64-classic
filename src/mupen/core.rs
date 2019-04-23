use crate::platform::{DEFAULT_DYNLIB, LIB_SEARCH_DIRS, PLUGIN_SEARCH_DIRS};

use super::lib::{M64pCommand, M64pError, M64pPluginType, Mupen64Plus, MupenError, Version};
use super::plugin::MupenPlugin;

use libc::{c_char, c_int, c_void};
use libloading::Library;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

static SM64_ROM: &[u8] = include_bytes!("../../Super Mario 64 (USA).n64");

pub struct MupenCore {
    lib: Mupen64Plus,
    plugins: Plugins,
    version: Version,
    api_versions: ApiVersions,
}

#[derive(Debug)]
struct Plugins {
    audio: MupenPlugin,
    input: MupenPlugin,
    rsp: MupenPlugin,
    video: MupenPlugin,
}

#[derive(Debug)]
struct ApiVersions {
    config_version: c_int,
    debug_version: c_int,
    vidext_version: c_int,
    extra_version: c_int,
}

impl MupenCore {
    pub fn new() -> Self {
        let lib = Self::load_lib().expect("MupenCore::load_lib() failed");
        let plugins = Self::load_plugins(&lib).expect("MupenCore::load_plugins() failed");
        let version = Self::get_core_version(&lib).expect("MupenCore::get_core_version() failed");
        let api_versions =
            Self::get_api_versions(&lib).expect("MupenCore::get_api_versions() failed");
        dbg!(&version);
        dbg!(&api_versions);
        Self {
            lib,
            plugins,
            version,
            api_versions,
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

    fn load_plugins(lib: &Mupen64Plus) -> Result<Plugins, io::Error> {
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
            audio: MupenPlugin::new(lib, Library::new(audio_plugin?).unwrap().into()),
            input: MupenPlugin::new(lib, Library::new(input_plugin?).unwrap().into()),
            rsp: MupenPlugin::new(lib, Library::new(rsp_plugin?).unwrap().into()),
            video: MupenPlugin::new(lib, Library::new(video_plugin?).unwrap().into()),
        });
    }

    fn get_core_version(lib: &Mupen64Plus) -> Result<Version, MupenError> {
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
            _ => Err(MupenError::new(lib, m64p_error)),
        }
    }

    fn get_api_versions(lib: &Mupen64Plus) -> Result<ApiVersions, MupenError> {
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
            M64pError::Success => Ok(ApiVersions {
                config_version,
                debug_version,
                vidext_version,
                extra_version,
            }),
            _ => Err(MupenError::new(&lib, m64p_error)),
        }
    }

    pub fn startup(&self) -> Result<(), MupenError> {
        let config_path = std::ptr::null() as *const c_char;
        let data_path = std::ptr::null() as *const c_char;
        let context = std::ptr::null() as *const c_void;
        let debug_callback = None;
        let context2 = std::ptr::null() as *const c_void;
        let state_callback = std::ptr::null();
        unsafe {
            let m64p_error = self.lib.core_startup(
                self.version.api_version,
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
            }?;
        }
        self.startup_plugins()
    }

    fn startup_plugins(&self) -> Result<(), MupenError> {
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
            .startup(&self.lib, context, debug_callback);
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
